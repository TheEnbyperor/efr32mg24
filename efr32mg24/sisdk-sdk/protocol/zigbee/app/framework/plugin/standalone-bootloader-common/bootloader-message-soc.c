/***************************************************************************//**
 * @file
 * @brief This file defines the interface to the host to send Ember proprietary
 * bootloader messages.
 *******************************************************************************
 * # License
 * <b>Copyright 2018 Silicon Laboratories Inc. www.silabs.com</b>
 *******************************************************************************
 *
 * The licensor of this software is Silicon Laboratories Inc. Your use of this
 * software is governed by the terms of Silicon Labs Master Software License
 * Agreement (MSLA) available at
 * www.silabs.com/about-us/legal/master-software-license-agreement. This
 * software is distributed to you in Source Code format and is governed by the
 * sections of the MSLA applicable to Source Code.
 *
 ******************************************************************************/

#include "app/framework/include/af.h"
#include "bootloader-protocol.h"

#define sl_zigbee_bootload_transmit_complete_handler sli_zigbee_af_standalone_bootloader_common_bootload_transmit_complete_callback
#define sl_zigbee_incoming_bootload_message_handler sli_zigbee_af_standalone_bootloader_common_incoming_bootload_message_callback

//------------------------------------------------------------------------------
// External Declarations

void sli_zigbee_aes_encrypt(uint8_t *block, const uint8_t *key);

//------------------------------------------------------------------------------
// Globals

//------------------------------------------------------------------------------
// Functions

sl_status_t sli_zigbee_af_send_bootload_message(bool isBroadcast,
                                                sl_802154_long_addr_t destEui64,
                                                uint8_t length,
                                                uint8_t* message)
{
  return sl_zigbee_send_bootload_message(isBroadcast,
                                         destEui64,
                                         length,
                                         message);
}

void sl_zigbee_incoming_bootload_message_handler(sl_802154_long_addr_t longId,
                                                 sl_zigbee_rx_packet_info_t *packetInfo,
                                                 uint8_t messageLength,
                                                 uint8_t *messageContents)
{
  if (messageLength > MAX_BOOTLOAD_MESSAGE_SIZE) {
    bootloadPrintln("Bootload message too long (%d > %d), dropping!",
                    messageLength,
                    MAX_BOOTLOAD_MESSAGE_SIZE);
    return;
  }
  sl_zigbee_af_standalone_bootloader_common_incoming_message_cb(longId,
                                                                messageLength,
                                                                messageContents);
}

void sl_zigbee_bootload_transmit_complete_handler(sl_status_t status,
                                                  uint8_t messageLength,
                                                  uint8_t *messageContents)
{
  if (status != SL_STATUS_OK) {
    uint8_t commandId = 0xFF;
    if (messageLength >= 2) {
      commandId = messageContents[1];
    }
    bootloadPrintln("Bootload message (0x%X) send failed: 0x%X",
                    commandId,
                    status);
  }
}

void sli_zigbee_af_standalone_bootloader_client_encrypt(uint8_t* block, uint8_t* key)
{
  sli_zigbee_aes_encrypt(block, key);
}
