/***************************************************************************//**
 * @file
 * @brief  The Tunnel Manager is responsible for establishing and maintaining tunnels
 * to all devices.  There are four APIs exposed by the tunnel manager. The
 * init function is called at startup and initializes internal data structures.
 * The create function is called after the CBKE with the device, the sendData
 * function is called whenever data is to be sent to the device, and the
 * destroy function is called whenever the tunnel to the device should
 * be torn down. There are also 1 callback that the tunnel manager will call.
 * It is sli_zigbee_af_comms_hub_function_tunnel_data_received_callback which is
 * called when data is received from a tunnel.
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

#ifndef TUNNEL_MANAGER_H_
#define TUNNEL_MANAGER_H_

#define SL_ZIGBEE_AF_PLUGIN_COMMS_HUB_FUNCTION_TUNNEL_LIMIT \
  (SL_ZIGBEE_AF_PLUGIN_TUNNELING_CLIENT_TUNNEL_LIMIT + SL_ZIGBEE_AF_PLUGIN_TUNNELING_SERVER_TUNNEL_LIMIT)
#define EM_AF_PLUGIN_COMMS_HUB_FUNCTION_NULL_TUNNEL_INDEX 0xFF

/**
 * @brief Initializes internal data structures.
 *
 */
void sli_zigbee_af_comms_hub_function_tunnel_init(uint8_t init_level);

/**
 * @brief Creates a tunnel to a device of type ESME, HCALCS or PPMID.
 *
 * As defined in section 10.2.2.1 of the GBCS version 0.8 and copied below, the
 * CHF will call this function to request a tunnel be created with the specified
 * destination.
 *
 * "When a Communications Hub has successfully established a shared secret key
 * using CBKE with a Device of type ESME, HCALCS or PPMID, the CHF shall send a
 * RequestTunnel command to the Device to request a tunnel association with the
 * Device.
 *
 * Where an ESME, a HCALCS or a PPMID remains in the CHF Device Log, the CHF
 * shall send a RequestTunnel command to the Device whenever:
 *  -  0xFFFF seconds have elapsed since receipt of the most recent
 *     RequestTunnelResponse command from that Device; or
 *  -  the CHF receives a Remote Party Message addressed to the Device but does
 *     not have a functioning tunnel association with the Device; or
 *  -  the CHF powers on.
 *
 *  Where the CHF receives a RequestTunnelResponse command from a Device with a
 *  TunnelStatus of 0x01 (Busy), the CHF shall send another RequestTunnel
 *  command three minutes later.
 *
 *  Where the CHF receives a RequestTunnelResponse command from a Device with a
 *  TunnelStatus of 0x02 (No More Tunnel IDs), the CHF shall send a CloseTunnel
 *  command for any TunnelID that may relate to an active tunnel association
 *  with that Device and, after receiving responses to all such commands, send
 *  another RequestTunnel command."
 *
 * @param remoteDeviceId The EUI64 of the device to which a tunnel is to be created
 * @param remoteEndpoint The remote endpoint to which the tunnel is to be created
 * @return true if successful or false if an error occurred
 */
bool sli_zigbee_af_comms_hub_function_tunnel_create(sl_802154_long_addr_t remoteDeviceId,
                                                    uint8_t remoteEndpoint);

/**
 * @brief Transfers data to a server through a tunnel.
 *
 * This function is used to transfer data to a server through a tunnel.
 *
 * @param remoteDeviceId The EUI64 of the device to which data is to be sent.
 * @param headerLen The length of any header that may prefix data.
 * @param header The buffer containing the raw octets of the header.
 * @param dataLen The length in octets of data.
 * @param data The buffer containing the raw octets of data.
 * @return True if successful or false if an error occurred.
 */
bool sli_zigbee_af_comms_hub_function_tunnel_send_data(sl_802154_long_addr_t remoteDeviceId,
                                                       uint16_t headerLen,
                                                       uint8_t *header,
                                                       uint16_t dataLen,
                                                       uint8_t *data);

/**
 * @brief Closes a tunnel.
 *
 * This function is used to close a tunnel.
 *
 * @param remoteDeviceId The EUI64 of the device to which the tunnel will be closed
 * @return True if successful or false if an error occurred.
 */
bool sli_zigbee_af_comms_hub_function_tunnel_destroy(sl_802154_long_addr_t remoteDeviceId);

/**
 * @brief Cleans up a tunnel.
 *
 * This function is used to clean up all states associated with a tunnel.
 *
 * @param remoteDeviceId The EUI64 of the device to which the tunnel state
 * is to be cleaned up.
 */
void sli_zigbee_af_comms_hub_function_tunnel_cleanup(sl_802154_long_addr_t remoteDeviceId);

/**
 * @brief Closes a tunnel.
 *
 * This function is used to close a tunnel on the commshub.
 *
 * @param remoteDeviceId The EUI64 of the device whose tunnel is to be closed.
 */
void sli_zigbee_af_comms_hub_function_tunnel_close(sl_802154_long_addr_t remoteDeviceId);

/**
 * @brief Prints the tunnel table.
 *
 * This function is used to print the tunneling table of the commshub.
 *
 */
void sli_zigbee_af_comms_hub_function_print(void);

bool sli_zigbee_af_comms_hub_function_tunnel_exists(sl_802154_long_addr_t deviceEui64);

#endif /* TUNNEL_MANAGER_H_ */
