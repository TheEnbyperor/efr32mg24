/***************************************************************************//**
 * @file high_datarate_phy_stack_interface_ipc_command_messages.c
 * @brief internal wrappers for 'high_datarate_phy_stack_interface' ipc commands
 *******************************************************************************
 * # License
 * <b>Copyright 2024 Silicon Laboratories Inc. www.silabs.com</b>
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
// automatically generated from high_datarate_phy_stack_interface.h.  Do not manually edit
#include "stack/include/high_datarate_phy_stack_interface.h"
#include "stack/internal/inc/high_datarate_phy_stack_interface_internal_def.h"
#include "stack/internal/src/ipc/high_datarate_phy_stack_interface_ipc_command_messages.h"
#include "stack/internal/src/ipc/zigbee_ipc_command_messages.h"

// ipc command dispatch

void sli_mac_stack_lower_mac_set_high_datarate_csma_params_process_ipc_command(sli_zigbee_ipc_cmd_t *msg)
{
  sli_mac_stack_lower_mac_set_high_datarate_csma_params(&msg->data.lower_mac_set_high_datarate_csma_params.request.csma_params);
}

void sli_mac_stack_lower_mac_set_high_datarate_phy_radio_priorities_process_ipc_command(sli_zigbee_ipc_cmd_t *msg)
{
  sli_mac_stack_lower_mac_set_high_datarate_phy_radio_priorities(&msg->data.lower_mac_set_high_datarate_phy_radio_priorities.request.priorities);
}

void sli_mac_stack_send_raw_high_datarate_phy_message_process_ipc_command(sli_zigbee_ipc_cmd_t *msg)
{
  msg->data.send_raw_high_datarate_phy_message.response.result = sli_mac_stack_send_raw_high_datarate_phy_message(msg->data.send_raw_high_datarate_phy_message.request.nwk_index,
                                                                                                                  msg->data.send_raw_high_datarate_phy_message.request.payload);
}

void sli_mac_stack_set_mode_switch_sync_detect_process_ipc_command(sli_zigbee_ipc_cmd_t *msg)
{
  msg->data.set_mode_switch_sync_detect.response.result = sli_mac_stack_set_mode_switch_sync_detect(msg->data.set_mode_switch_sync_detect.request.enable_f);
}

// public entrypoints

void sl_mac_lower_mac_set_high_datarate_csma_params(RAIL_CsmaConfig_t *csma_params)
{
  sli_zigbee_ipc_cmd_t msg;

  if (csma_params != NULL) {
    msg.data.lower_mac_set_high_datarate_csma_params.request.csma_params = *csma_params;
  }

  sli_zigbee_send_ipc_cmd(sli_mac_stack_lower_mac_set_high_datarate_csma_params_process_ipc_command, &msg);

  if (csma_params != NULL) {
    *csma_params = msg.data.lower_mac_set_high_datarate_csma_params.request.csma_params;
  }
}

void sl_mac_lower_mac_set_high_datarate_phy_radio_priorities(sl_zigbee_multiprotocol_priorities_t *priorities)
{
  sli_zigbee_ipc_cmd_t msg;

  if (priorities != NULL) {
    msg.data.lower_mac_set_high_datarate_phy_radio_priorities.request.priorities = *priorities;
  }

  sli_zigbee_send_ipc_cmd(sli_mac_stack_lower_mac_set_high_datarate_phy_radio_priorities_process_ipc_command, &msg);

  if (priorities != NULL) {
    *priorities = msg.data.lower_mac_set_high_datarate_phy_radio_priorities.request.priorities;
  }
}

sl_status_t sl_mac_send_raw_high_datarate_phy_message(uint8_t nwk_index,
                                                      uint8_t *payload)
{
  sli_zigbee_ipc_cmd_t msg;
  msg.data.send_raw_high_datarate_phy_message.request.nwk_index = nwk_index;

  if ((((payload[1] << 8) + payload[0] + 2)) > (MAX_HIGH_DATARATE_PHY_PACKET_LENGTH)) {
    assert(false); // "vector payload length exceeds expected maximum
  }

  memmove(msg.data.send_raw_high_datarate_phy_message.request.payload, payload, sizeof(uint8_t) * (((payload[1] << 8) + payload[0] + 2)));
  sli_zigbee_send_ipc_cmd(sli_mac_stack_send_raw_high_datarate_phy_message_process_ipc_command, &msg);

  if ((((payload[1] << 8) + payload[0] + 2)) > (MAX_HIGH_DATARATE_PHY_PACKET_LENGTH)) {
    assert(false); // "vector payload length exceeds expected maximum
  }

  memmove(payload, msg.data.send_raw_high_datarate_phy_message.request.payload, sizeof(uint8_t) * (((payload[1] << 8) + payload[0] + 2)));
  return msg.data.send_raw_high_datarate_phy_message.response.result;
}

RAIL_Status_t sl_mac_set_mode_switch_sync_detect(bool enable_f)
{
  sli_zigbee_ipc_cmd_t msg;
  msg.data.set_mode_switch_sync_detect.request.enable_f = enable_f;
  sli_zigbee_send_ipc_cmd(sli_mac_stack_set_mode_switch_sync_detect_process_ipc_command, &msg);

  return msg.data.set_mode_switch_sync_detect.response.result;
}
