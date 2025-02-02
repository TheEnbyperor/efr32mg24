/***************************************************************************//**
 * @file
 * @brief Plugin for receiving InterPAN messages.
 * See @ref message for documentation.
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

#ifndef SILABS_ZIGBEE_INTERPAN_H
#define SILABS_ZIGBEE_INTERPAN_H

#include "interpan-config.h"
#if (SL_ZIGBEE_AF_PLUGIN_INTERPAN_ALLOW_REQUIRED_SMART_ENERGY_MESSAGES == 1)
#define ALLOW_REQUIRED_SMART_ENERGY_MESSAGES
#endif
#if (SL_ZIGBEE_AF_PLUGIN_INTERPAN_ALLOW_SMART_ENERGY_RESPONSE_MESSAGES == 1)
#define ALLOW_SMART_ENERGY_RESPONSE_MESSAGES
#endif
#if (SL_ZIGBEE_AF_PLUGIN_INTERPAN_ALLOW_KEY_ESTABLISHMENT == 1)
#define ALLOW_KEY_ESTABLISHMENT
#endif
#if (SL_ZIGBEE_AF_PLUGIN_INTERPAN_ALLOW_APS_ENCRYPTED_MESSAGES == 1)
#define ALLOW_APS_ENCRYPTED_MESSAGES
#endif
#if (SL_ZIGBEE_AF_PLUGIN_INTERPAN_ALLOW_FRAGMENTATION == 1)
#define ALLOW_FRAGMENTATION
#endif

/**
 * @defgroup interpan Interpan
 * @ingroup component
 * @brief API and Callbacks for the Interpan Component
 *
 * A component used to handle sending and receiving inter-PAN messages,
 * which are messages that are sent by unjoined devices 1-hop away.
 *
 */

/**
 * @addtogroup interpan
 * @{
 */

// The three types of inter-PAN messages.  The values are actually the
// corresponding APS frame controls.
//
// 0x03 is the special interPAN message type.  Unicast mode is 0x00,
// broadcast mode is 0x08, and multicast mode is 0x0C.
//

#include  "mac-types.h"
// MAC Frame Max size
// - Frame control   (2-bytes)
// - Sequence        (1-byte)
// - Dest PAN ID     (2-bytes)
// - Dest long       (8-bytes)
// - Source PAN ID   (2-bytes)
// - Source long     (8-bytes)
#define MAX_INTER_PAN_MAC_SIZE 23

// NWK stub frame has two control bytes.
#define STUB_NWK_SIZE 2
#define STUB_NWK_FRAME_CONTROL 0x000B

// Interpan APS Unicast
//  - Frame Control   (1-byte)
//  - Cluster ID      (2-bytes)
//  - Profile ID      (2-bytes)
#define INTERPAN_APS_UNICAST_SIZE 5

// Interpan APS Broadcast, same as unicast
#define INTERPAN_APS_BROADCAST_SIZE 5

// Interpan APS Multicast
//  - Frame Control   (1-byte)
//  - Group ID        (2-bytes)
//  - Cluster ID      (2-bytes)
//  - Profile ID      (2-bytes)
#define INTERPAN_APS_MULTICAST_SIZE  7

#define MAX_STUB_APS_SIZE (INTERPAN_APS_MULTICAST_SIZE)
#define MIN_STUB_APS_SIZE (INTERPAN_APS_UNICAST_SIZE)

#define INTERPAN_UNICAST_HEADER_SIZE \
  (MAX_INTER_PAN_MAC_SIZE + STUB_NWK_SIZE + INTERPAN_APS_UNICAST_SIZE)

#define INTERPAN_MULTICAST_HEADER_SIZE \
  (MAX_INTER_PAN_MAC_SIZE + STUB_NWK_SIZE + INTERPAN_APS_MULTICAST_SIZE)

#define MAX_INTER_PAN_HEADER_SIZE   (INTERPAN_MULTICAST_HEADER_SIZE)

#define INTERPAN_APS_FRAME_TYPE      0x03
#define INTERPAN_APS_FRAME_TYPE_MASK 0x03

// The only allowed APS FC value (without the delivery mode subfield)
#define INTERPAN_APS_FRAME_CONTROL_NO_DELIVERY_MODE (INTERPAN_APS_FRAME_TYPE)

#define INTERPAN_APS_FRAME_DELIVERY_MODE_MASK 0x0C
#define INTERPAN_APS_FRAME_SECURITY           0x20

// 5 byte AUX header + 4 byte MIC
#define INTERPAN_APS_ENCRYPTION_OVERHEAD      (5 + 4)

// Control byte (1), Index IPMF (1), Number IPMF (1), Len IPMF (1)
#define SL_ZIGBEE_APS_INTERPAN_FRAGMENTATION_OVERHEAD           4
// Control byte (1), Index IPMF (1), Response (1)
#define SL_ZIGBEE_APS_INTERPAN_FRAGMENTATION_RESPONSE_LEN       3
#define SL_ZIGBEE_APS_INTERPAN_FRAGMENT_MIN_LEN \
  SL_ZIGBEE_APS_INTERPAN_FRAGMENTATION_RESPONSE_LEN

#define INTERPAN_FRAGMENTATION_APS_CONTROL_BYTE_INDEX   0
#define INTERPAN_FRAGMENTATION_APS_INDEX_IPMF_INDEX     1
#define INTERPAN_FRAGMENTATION_APS_IPMF_RESPONSE_INDEX  2 // IPMF response only
#define INTERPAN_FRAGMENTATION_APS_LEN_IPMF_INDEX       3 // IPMF only
#define INTERPAN_FRAGMENTATION_APS_CONTROL_BYTE_IPMF_VAL            0x00
#define INTERPAN_FRAGMENTATION_APS_CONTROL_BYTE_IPMF_RESPONSE_VAL   0x80

#define INTERPAN_IPMF_RESPONSE_SUCCESS    0x00
#define INTERPAN_IPMF_RESPONSE_FAILURE    0x01

#define SL_ZIGBEE_AF_PLUGIN_INTERPAN_FRAGMENTATION_MAX_PAYLOAD_SIZE   1500

// NOTE: unlike regular frags, which rely on APS sequence numbers in the ACK to
// know when to send the next fragment, interpan frags do not have APS ACKs.
// The only way to distinguish is to judge based on the partner ID. For any
// partner, only 1 packet can be transmitted to or received from at a time
#define SL_ZIGBEE_AF_PLUGIN_INTERPAN_FRAGMENTATION_MAX_INCOMING_PACKETS 1
#define SL_ZIGBEE_AF_PLUGIN_INTERPAN_FRAGMENTATION_MAX_OUTGOING_PACKETS 1
#define SL_ZIGBEE_AF_PLUGIN_INTERPAN_FRAGMENTATION_BUFFER_SIZE \
  (SL_ZIGBEE_AF_PLUGIN_INTERPAN_FRAGMENTATION_MAX_PAYLOAD_SIZE + INTERPAN_UNICAST_HEADER_SIZE)

#define SL_ZIGBEE_AF_PLUGIN_INTERPAN_FILTER_LIST    \
  (SL_802154_FILTER_MATCH_ENABLED                   \
   | SL_802154_FILTER_MATCH_ON_PAN_DEST_LOCAL       \
   | SL_802154_FILTER_MATCH_ON_PAN_SOURCE_NON_LOCAL \
   | SL_802154_FILTER_MATCH_ON_DEST_BROADCAST_SHORT \
   | SL_802154_FILTER_MATCH_ON_SOURCE_LONG),        \
  (SL_802154_FILTER_MATCH_ENABLED                   \
   | SL_802154_FILTER_MATCH_ON_PAN_DEST_LOCAL       \
   | SL_802154_FILTER_MATCH_ON_PAN_SOURCE_NON_LOCAL \
   | SL_802154_FILTER_MATCH_ON_DEST_UNICAST_SHORT   \
   | SL_802154_FILTER_MATCH_ON_SOURCE_LONG),        \
  (SL_802154_FILTER_MATCH_ENABLED                   \
   | SL_802154_FILTER_MATCH_ON_PAN_DEST_LOCAL       \
   | SL_802154_FILTER_MATCH_ON_PAN_SOURCE_NON_LOCAL \
   | SL_802154_FILTER_MATCH_ON_DEST_UNICAST_LONG    \
   | SL_802154_FILTER_MATCH_ON_SOURCE_LONG),        \
  (SL_802154_FILTER_MATCH_ENABLED                   \
   | SL_802154_FILTER_MATCH_ON_PAN_DEST_LOCAL       \
   | SL_802154_FILTER_MATCH_ON_PAN_SOURCE_LOCAL     \
   | SL_802154_FILTER_MATCH_ON_DEST_BROADCAST_SHORT \
   | SL_802154_FILTER_MATCH_ON_SOURCE_LONG),        \
  (SL_802154_FILTER_MATCH_ENABLED                   \
   | SL_802154_FILTER_MATCH_ON_PAN_DEST_LOCAL       \
   | SL_802154_FILTER_MATCH_ON_PAN_SOURCE_LOCAL     \
   | SL_802154_FILTER_MATCH_ON_DEST_UNICAST_SHORT   \
   | SL_802154_FILTER_MATCH_ON_SOURCE_LONG),        \
  (SL_802154_FILTER_MATCH_ENABLED                   \
   | SL_802154_FILTER_MATCH_ON_PAN_DEST_LOCAL       \
   | SL_802154_FILTER_MATCH_ON_PAN_SOURCE_LOCAL     \
   | SL_802154_FILTER_MATCH_ON_DEST_UNICAST_LONG    \
   | SL_802154_FILTER_MATCH_ON_SOURCE_LONG),        \
  (SL_802154_FILTER_MATCH_ENABLED                   \
   | SL_802154_FILTER_MATCH_ON_PAN_DEST_BROADCAST   \
   | SL_802154_FILTER_MATCH_ON_PAN_SOURCE_NON_LOCAL \
   | SL_802154_FILTER_MATCH_ON_DEST_BROADCAST_SHORT \
   | SL_802154_FILTER_MATCH_ON_SOURCE_LONG),        \
  (SL_802154_FILTER_MATCH_ENABLED                   \
   | SL_802154_FILTER_MATCH_ON_PAN_DEST_BROADCAST   \
   | SL_802154_FILTER_MATCH_ON_PAN_SOURCE_NON_LOCAL \
   | SL_802154_FILTER_MATCH_ON_DEST_UNICAST_SHORT   \
   | SL_802154_FILTER_MATCH_ON_SOURCE_LONG),        \
  (SL_802154_FILTER_MATCH_ENABLED                   \
   | SL_802154_FILTER_MATCH_ON_PAN_DEST_BROADCAST   \
   | SL_802154_FILTER_MATCH_ON_PAN_SOURCE_NON_LOCAL \
   | SL_802154_FILTER_MATCH_ON_DEST_UNICAST_LONG    \
   | SL_802154_FILTER_MATCH_ON_SOURCE_LONG),        \
  (SL_802154_FILTER_MATCH_ENABLED                   \
   | SL_802154_FILTER_MATCH_ON_PAN_DEST_BROADCAST   \
   | SL_802154_FILTER_MATCH_ON_PAN_SOURCE_LOCAL     \
   | SL_802154_FILTER_MATCH_ON_DEST_BROADCAST_SHORT \
   | SL_802154_FILTER_MATCH_ON_SOURCE_LONG),        \
  (SL_802154_FILTER_MATCH_ENABLED                   \
   | SL_802154_FILTER_MATCH_ON_PAN_DEST_BROADCAST   \
   | SL_802154_FILTER_MATCH_ON_PAN_SOURCE_LOCAL     \
   | SL_802154_FILTER_MATCH_ON_DEST_UNICAST_SHORT   \
   | SL_802154_FILTER_MATCH_ON_SOURCE_LONG),        \
  (SL_802154_FILTER_MATCH_ENABLED                   \
   | SL_802154_FILTER_MATCH_ON_PAN_DEST_BROADCAST   \
   | SL_802154_FILTER_MATCH_ON_PAN_SOURCE_LOCAL     \
   | SL_802154_FILTER_MATCH_ON_DEST_UNICAST_LONG    \
   | SL_802154_FILTER_MATCH_ON_SOURCE_LONG),        \

#if defined(ALLOW_FRAGMENTATION)

#define UNUSED_TX_PACKET_ENTRY 0xFF
typedef struct {
  sl_zigbee_outgoing_message_type_t  messageType;
  sl_802154_long_addr_t                destEui;
  uint8_t                   buffer[SL_ZIGBEE_AF_PLUGIN_INTERPAN_FRAGMENTATION_BUFFER_SIZE]; // Whole message, including hdrs
  uint16_t                  bufLen;             // Whole message size
  uint8_t                   fragmentMaxLen;     // Max possible frag length
  uint8_t                   numFragments;       // Number of frags to be sent
  uint8_t                   fragmentNum;        // Running # of which frag is TX
  sl_zigbee_af_event_t*        event;
} sli_zigbee_tx_fragmented_interpan_packet_t;

typedef struct {
  sl_802154_long_addr_t           sourceEui;
  uint8_t              buffer[SL_ZIGBEE_AF_PLUGIN_INTERPAN_FRAGMENTATION_BUFFER_SIZE];
  uint16_t             bufLen;
  uint8_t              numFragments;
  uint8_t              lastFragmentNumReceived;
  sl_zigbee_af_event_t*   event;
} sli_zigbee_rx_fragmented_interpan_packet_t;

void interpanPluginSetFragmentMessageTimeout(uint16_t timeout); // seconds

#endif // SL_ZIGBEE_AF_PLUGIN_INTERPAN_ALLOW_FRAGMENTATION

typedef enum {
  IPMF_SUCCESS = 0,
  IPMF_TX_TIMEOUT,        // IPMF response not received
  IPMF_RX_TIMEOUT,        // Next IPMF not received
  IPMF_TX_BAD_RESPONSE,   // Bad IPMF header data
  IPMF_RX_BAD_RESPONSE    // Bad IPMF header data
} sli_zigbee_e_interpan_fragmentation_status_t;

/**
 * @name API
 * @{
 */

/** @} */ // end of name API

/**
 * @name Callbacks
 * @{
 */

/**
 * @defgroup interpan_cb Interpan
 * @ingroup af_callback
 * @brief Callbacks for Interpan Component
 *
 */

/**
 * @addtogroup interpan_cb
 * @{
 */

/** @brief Called by the Interpan plugin when an interpan message is
 * received but has not yet been handled by the plugin or the framework.
 *
 * The application should return true if the message was handled.
 *
 * @param header The inter-PAN header Ver.: always
 * @param msgLen The message payload length Ver.: always
 * @param message The message payload Ver.: always
 */
bool sl_zigbee_af_interpan_pre_message_received_cb(const sl_zigbee_af_interpan_header_t *header,
                                                   uint8_t msgLen,
                                                   uint8_t *message);

/** @brief Message received over fragments.
 *
 * This function is called by the Interpan plugin when a fully reconstructed
 * message has been received over inter-PAN fragments, or IPMFs.
 *
 * @param header The inter-PAN header Ver.: always
 * @param msgLen The message payload length Ver.: always
 * @param message The message payload Ver.: always
 */
void sl_zigbee_af_interpan_message_received_over_fragments_cb(const sl_zigbee_af_interpan_header_t *header,
                                                              uint8_t msgLen,
                                                              uint8_t *message);

/** @brief Fragment transmission failed.
 *
 * This function is called by the Interpan plugin when a fragmented
 * transmission has failed.
 *
 * @param interpanFragmentationStatus The status describing why transmission
 * failed Ver.: always
 * @param fragmentNum The fragment number that encountered the failure
 * Ver.: always
 */
void sl_zigbee_af_interpan_fragment_transmission_failed_cb(uint8_t interpanFragmentationStatus,
                                                           uint8_t fragmentNum);

/** @} */ // end of interpan_cb
/** @} */ // end of name Callbacks
/** @} */ // end of interpan

void interpanPluginInit(uint8_t init_level);
void interpanPluginSetMacMatchFilterEnable(bool enable);

bool sli_zigbee_af_interpan_process_message(uint8_t messageLength,
                                            uint8_t *messageContents);

sl_status_t sli_zigbee_af_interpan_send_raw_message(uint8_t length, uint8_t* message);

sl_status_t sli_zigbee_af_interpan_aps_crypt_message(bool encrypt,
                                                     uint8_t* apsFrame,
                                                     uint8_t* messageLength,
                                                     uint8_t apsHeaderLength,
                                                     sl_802154_long_addr_t remoteEui64);

void sli_zigbee_af_interpan_set_enable_state(bool enable);

#endif // SILABS_ZIGBEE_INTERPAN_H
