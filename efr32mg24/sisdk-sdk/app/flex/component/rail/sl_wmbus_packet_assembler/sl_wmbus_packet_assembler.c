/***************************************************************************//**
 * @file  sl_wmbus_packet_assembler.c
 * @brief wMBus packet assembler implementation.
 *******************************************************************************
 * # License
 * <b>Copyright 2024 Silicon Laboratories Inc. www.silabs.com</b>
 *******************************************************************************
 *
 * SPDX-License-Identifier: Zlib
 *
 * The licensor of this software is Silicon Laboratories Inc.
 *
 * This software is provided 'as-is', without any express or implied
 * warranty. In no event will the authors be held liable for any damages
 * arising from the use of this software.
 *
 * Permission is granted to anyone to use this software for any purpose,
 * including commercial applications, and to alter it and redistribute it
 * freely, subject to the following restrictions:
 *
 * 1. The origin of this software must not be misrepresented; you must not
 *    claim that you wrote the original software. If you use this software
 *    in a product, an acknowledgment in the product documentation would be
 *    appreciated but is not required.
 * 2. Altered source versions must be plainly marked as such, and must not be
 *    misrepresented as being the original software.
 * 3. This notice may not be removed or altered from any source distribution.
 *
 ******************************************************************************/

// -----------------------------------------------------------------------------
//                                   Includes
// -----------------------------------------------------------------------------

#include "sl_wmbus_packet_assembler.h"
#include <string.h>

// -----------------------------------------------------------------------------
//                              Macros and Typedefs
// -----------------------------------------------------------------------------

#define SL_WMBUS_DEV_ID 0x01  //BCD

#define SL_WMBUS_VERSION 0x01

// -----------------------------------------------------------------------------
//                          Static Function Declarations
// -----------------------------------------------------------------------------

/*******************************************************************************
 * @brief Sets up the DLL header in the given buffer.
 *
 * Setting up the DLL header in the provided buffer. The DLL header includes
 * information such as whether the frame is Frame B, the frame length,
 * the function code, and various flags.
 *
 * @param buffer The buffer to set up the DLL header in.
 * @param is_frame_b Indicates whether the frame is Frame B.
 * @param frame_length The length of the frame.
 * @param function_code The function code.
 * @param primary Indicates whether the frame is primary.
 * @param fcv_dcf Indicates whether the frame has FCV/DCF.
 * @param fcb_acd Indicates whether the frame has FCB/ACD.
 ******************************************************************************/
static void sli_setup_dll_header(void *buffer, bool is_frame_b, uint16_t frame_length, WMBUS_function_code_t function_code, bool primary, bool fcv_dcf, bool fcb_acd);

/*******************************************************************************
 * @brief Sets up the STL header for the WMBus packet assembler.
 *
 * Setting up the STL header in the provided buffer. The STL header includes
 * information such as the mode, CI field, access number, accessibility,
 * synchronization flag, payload, and payload length.
 *
 * @param buffer The buffer to store the STL header.
 * @param mode The mode of the WMBus packet.
 * @param ci_field The CI field of the WMBus packet.
 * @param access_number The access number of the WMBus packet.
 * @param accessibility The accessibility of the WMBus packet.
 * @param synchronized Flag indicating whether the packet is synchronized.
 * @param payload The payload of the WMBus packet.
 * @param payload_length The length of the payload.
 * @return The length of the STL header.
 ******************************************************************************/
static uint16_t sli_setup_stl_header(void *buffer, uint8_t mode, uint8_t ci_field, uint8_t access_number, WMBUS_accessibility_t accessibility, bool synchronized, void *payload, uint16_t payload_length);

// -----------------------------------------------------------------------------
//                                Global Variables
// -----------------------------------------------------------------------------

// -----------------------------------------------------------------------------
//                                Static Variables
// -----------------------------------------------------------------------------

static const char manuf[3] = { 'S', 'I', 'L' }; //FLAG assigned for Silicon Labs
static WMBUS_device_type_t device_type;

// -----------------------------------------------------------------------------
//                          Static Function Definitions
// -----------------------------------------------------------------------------

static void sli_setup_dll_header(void *buffer, bool is_frame_b, uint16_t frame_length, WMBUS_function_code_t function_code, bool primary, bool fcv_dcf, bool fcb_acd)
{
  WMBUS_dll_header_t *dll_header = (WMBUS_dll_header_t*)buffer;
  memset(dll_header, 0, sizeof(WMBUS_dll_header_t));//clear the header
  dll_header->c_field.detailed.primary = primary;
  dll_header->c_field.detailed.fcb_acd = fcb_acd;
  dll_header->c_field.detailed.fcv_dcf = fcv_dcf;
  dll_header->c_field.detailed.function_code = function_code;
  dll_header->address.detailed.manufacturer = WMBUSframe_Chars2MField(manuf);
  dll_header->address.detailed.id = SL_WMBUS_DEV_ID;
  dll_header->address.detailed.version = SL_WMBUS_VERSION;
  dll_header->address.detailed.device_type = device_type;

  frame_length -= 1; //l field shouldn't be counted
  if ( is_frame_b ) {
    if ( frame_length < 125 ) {
      dll_header->lField = (uint8_t)(frame_length + 2); //2block, 1 crc
    } else {
      dll_header->lField = (uint8_t)(frame_length + 4); //3block, 2 crc
    }
  } else {
    dll_header->lField = (uint8_t)frame_length;
  }
}

static uint16_t sli_setup_stl_header(void *buffer, uint8_t mode, uint8_t ci_field, uint8_t access_number, WMBUS_accessibility_t accessibility, bool synchronized, void *payload, uint16_t payload_length)
{
  (void) ci_field;
  WMBUS_stl_header_t *stl_header = (WMBUS_stl_header_t*)buffer;
  static const uint32_t dev_id = SL_WMBUS_DEV_ID;
  static const uint8_t version = SL_WMBUS_VERSION;

  memset(stl_header, 0, sizeof(WMBUS_stl_header_t));//clear the header
  stl_header->ci_field = WMBUS_CI_EN13757_3_APPLICATION_SHORT;
  stl_header->access_number = access_number;
  stl_header->conf_word.mode_0_5.mode = mode;
  stl_header->conf_word.mode_0_5.accessibility = accessibility;
  stl_header->conf_word.mode_0_5.synchronized = synchronized;
  if ( mode == 5 ) {
    uint8_t iv[16];
    uint16_t mf = WMBUSframe_Chars2MField(manuf);
    //with long transport layer header, the address from the header should be used
    memcpy(iv, &mf, 2);
    memcpy(iv + 2, &dev_id, 4);
    memcpy(iv + 6, &version, 1);
    memcpy(iv + 7, &device_type, 1);
    memset(iv + 8, access_number, 8);
    stl_header->conf_word.mode_0_5.num_of_enc_blocks = WMBUSframe_crypto5encrypt(payload, payload, iv, payload_length, 0);
    return 16 * stl_header->conf_word.mode_0_5.num_of_enc_blocks; //payload_length probably changed due to padding
  } else {
    return payload_length;
  }
}

// -----------------------------------------------------------------------------
//                          Public Function Definitions
// -----------------------------------------------------------------------------

/*******************************************************************************
 * Sets up a WMBus frame.
 ******************************************************************************/
uint16_t sl_wmbus_setup_frame(uint8_t *buffer, uint8_t access_number, WMBUS_accessibility_t accessibility, WMBUS_device_type_t new_device_type, sl_wmbus_sensor_data_t* sensors_data, bool periodic, bool encrypt)
{
  uint8_t *payload_ptr = buffer + sizeof(WMBUS_dll_header_t) + sizeof(WMBUS_stl_header_t);
  uint8_t *stl_ptr = buffer + sizeof(WMBUS_dll_header_t);
  uint8_t *dll_ptr = buffer;

  device_type = new_device_type;

  //setup the payload before the headers, so we'll know the length
  uint16_t frame_length = 0;
  if ( encrypt ) { //2B AES check
    frame_length += WMBUSframe_addIdleFiller(payload_ptr + frame_length, 2);
  }

  // Add all data to frame
  for (sl_wmbus_sensor_data_t* iter = sensors_data; iter != NULL; iter = iter->next) {
    frame_length += WMBUSframe_addPayloadI32(payload_ptr + frame_length, WMBUS_DIB_FUNCTION_INSTANTANEOUS, iter->vif, iter->data);
  }

  //continue with STL header, since encryption could change the length
  //replace length with the new length
  frame_length = sli_setup_stl_header(stl_ptr, encrypt ? 5 : 0, WMBUS_CI_EN13757_3_APPLICATION_SHORT, access_number,
                                      accessibility, periodic, payload_ptr, frame_length);

  //add the header lengths to frameLnegth
  frame_length += sizeof(WMBUS_dll_header_t) + sizeof(WMBUS_stl_header_t);
  //finally, set up the DLL header
  sli_setup_dll_header(dll_ptr, false, frame_length, WMBUS_FUNCTION_SND_NR, true, false, false);

  return frame_length;
}
