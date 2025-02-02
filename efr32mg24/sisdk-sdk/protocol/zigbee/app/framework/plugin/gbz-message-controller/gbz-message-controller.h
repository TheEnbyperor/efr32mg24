/***************************************************************************//**
 * @file
 * @brief APIs and defines for the GBCS Message Controller plugin.
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

#ifndef SILABS_GBZ_MESSAGE_CONTROLLER
#define SILABS_GBZ_MESSAGE_CONTROLLER

/**
 * @defgroup gbz-message-controller GBZ Message Controller
 * @ingroup component
 * @brief API and Callbacks for the GBZ Message Controller Component
 *
 * Silicon Labs implementation of a GBZ message controller. This component
 * extracts ZCL commands by parsing the elemental
 * GBZ components of the GBZ message. Processing of encrypted GBZ message
 * is not supported.
 *
 */

/**
 * @addtogroup gbz-message-controller
 * @{
 */

typedef enum {
  GCS06_MESSAGE_CODE  = 0x0070, // CHF
  GCS11_MESSAGE_CODE  = 0x0073, // CHF
  CS11_MESSAGE_CODE             = 0x0015,
  GCS01a_MESSAGE_CODE           = 0x006B,
  GCS01b_MESSAGE_CODE           = 0x00A3,
  GCS05_MESSAGE_CODE            = 0x006F,
  GCS07_MESSAGE_CODE            = 0x0071,
  GCS09_MESSAGE_CODE            = 0x0072,
  GCS13a_MESSAGE_CODE           = 0x0074,
  GCS13b_MESSAGE_CODE           = 0x00B8,
  GCS13c_MESSAGE_CODE           = 0x00B6,
  GCS14_MESSAGE_CODE            = 0x0075,
  GCS17_MESSAGE_CODE            = 0x0078,
  GCS21d_MESSAGE_CODE           = 0x009D,
  GCS21e_MESSAGE_CODE           = 0x009E,
  GCS21j_MESSAGE_CODE           = 0x00BF,
  GCS23_MESSAGE_CODE            = 0x007C,
  GCS25_MESSAGE_CODE            = 0x007E,
  GCS33_MESSAGE_CODE            = 0x0082,
  GCS38_MESSAGE_CODE            = 0x0084,
  GCS44_MESSAGE_CODE            = 0x0088,
  GCS46_MESSAGE_CODE            = 0x0089,
  GCS60_MESSAGE_CODE            = 0x008D,
  CS10a_MESSAGE_CODE            = 0x0014,
  CS10b_MESSAGE_CODE            = 0x00A1,
  GCS61_MESSAGE_CODE            = 0x00A0,
  GCS16a_MESSAGE_CODE           = 0x0077,
  GCS16b_MESSAGE_CODE           = 0x0096,
  GCS15b_MESSAGE_CODE           = 0x00C3,
  GCS15c_MESSAGE_CODE           = 0x0076,
  GCS15d_MESSAGE_CODE           = 0x00C4,
  GCS15e_MESSAGE_CODE           = 0x00C5,
  GCS21f_MESSAGE_CODE           = 0x009F,
  GCS21b_MESSAGE_CODE           = 0x00B5,
  GCS53_MESSAGE_CODE            = 0x008B,
  TEST_ENCRYPTED_MESSAGE_CODE   = 0xFFFE,
  TEST_MESSAGE_CODE             = 0xFFFF,
} sli_zigbee_gbcs_use_case_message_code_t;
// debug prints
#define sl_zigbee_af_gbz_message_controller_print(...)    sl_zigbee_af_app_print(__VA_ARGS__)
#define sl_zigbee_af_gbz_message_controller_println(...)  sl_zigbee_af_app_println(__VA_ARGS__)
#define sl_zigbee_af_gbz_message_controller_debug_exec(x)  sl_zigbee_af_app_debug_exec(x)
#define sl_zigbee_af_gbz_message_controller_print_buffer(buffer, len, withSpace) sl_zigbee_af_app_print_buffer(buffer, len, withSpace)

// offsets
#define GAS_PROXY_FUNCTION_GBZ_COMPONENT_EXT_HEADER_CONTROL_FIELD_OFFSET  (0)
#define GAS_PROXY_FUNCTION_GBZ_COMPONENT_EXT_HEADER_CLUSTER_ID_OFFSET     (2)
#define GAS_PROXY_FUNCTION_GBZ_COMPONENT_EXT_HEADER_GBZ_CMD_LENGTH_OFFSET (3)

#define GAS_PROXY_FUNCTION_GBZ_MESSAGE_COMMAND_HEADER_LENGTH              (3)
#define GAS_PROXY_FUNCTION_GBZ_MESSAGE_RESPONSE_HEADER_LENGTH             (3)
#define GAS_PROXY_FUNCTION_GBZ_MESSAGE_ALERT_HEADER_LENGTH                (9)
#define GAS_PROXY_FUNCTION_GBZ_COMPONENT_EXT_HEADER_FIELDS_LENGTH         (5)
#define GAS_PROXY_FUNCTION_GBZ_COMPONENT_ZCL_HEADER_LENGTH                (3)
#define GAS_PROXY_FUNCTION_GBZ_COMPONENT_FROM_DATE_TIME_LENGTH            (4)
#define GAS_PROXY_FUNCTION_GBZ_COMPONENT_ENCRYPTION_HEADER_FIELDS_LENGTH  (2)
#define GAS_PROXY_FUNCTION_GBZ_COMPONENT_ENCRYPTION_CIPHERED_INFO_LENGTH  (2)
#define GAS_PROXY_FUNCTION_GBZ_COMPONENT_EXT_HEADER_FIELDS_LENGTH         (5)
#define GAS_PROXY_FUNCTION_GBZ_COMPONENT_ENCRYPTION_HEADER_FIELDS_LENGTH  (2)
#define GAS_PROXY_FUNCTION_GBZ_COMPONENT_FROM_DATE_TIME_LENGTH            (4)
#define GAS_PROXY_FUNCTION_GBZ_COMPONENT_ENCRYPTION_CIPHERED_INFO_LENGTH  (2)
#define GAS_PROXY_FUNCTION_GBZ_COMPONENT_LAST_MSG_MASK       (0x01)
#define GAS_PROXY_FUNCTION_GBZ_COMPONENT_ENCRYPTED_MSG_MASK  (0x02)
#define GAS_PROXY_FUNCTION_GBZ_COMPONENT_FROM_DATE_TIME_MASK (0x10)

// Forward declaration
sl_zigbee_af_status_t sl_zigbee_af_cluster_specific_command_parse(sl_zigbee_af_cluster_command_t *cmd);

typedef enum {
  SL_ZIGBEE_AF_GBZ_NOT_LAST_UNENCRYPTED_MESSAGE = 0x00,
  SL_ZIGBEE_AF_GBZ_LAST_UNENCRYPTED_MESSAGE = 0x01,
  SL_ZIGBEE_AF_GBZ_NOT_LAST_ENCRYPTED_MESSAGE = 0x02,
  SL_ZIGBEE_AF_GBZ_LAST_ENCRYPTED_MESSAGE = 0x03,
} sl_zigbee_af_gbz_extended_header_control_field_t;

typedef enum {
  SL_ZIGBEE_AF_GBZ_MESSAGE_COMMAND,
  SL_ZIGBEE_AF_GBZ_MESSAGE_RESPONSE,
  SL_ZIGBEE_AF_GBZ_MESSAGE_ALERT
} sl_zigbee_af_gbz_message_type_t;

typedef struct {
  sl_zigbee_af_cluster_id_t  clusterId;
  uint8_t frameControl;
  uint8_t transactionSequenceNumber;
  uint8_t commandId;
  uint8_t * payload;
  uint16_t payloadLength;
  uint8_t direction;
  bool clusterSpecific;
  bool mfgSpecific;

  uint32_t fromDateTime;
  bool hasFromDateTime; // Zigbee UTC Time.
  bool encryption;
} sl_zigbee_af_gbz_zcl_command_t;

typedef struct {
  bool freeRequired;
  uint8_t * command;
  sl_zigbee_af_gbz_message_type_t type;
  uint16_t alertCode;
  uint32_t alertTimestamp;
  uint16_t profileId;
  uint8_t nextComponentZclSequenceNumber;
  uint8_t componentsSize;
  uint8_t componentsParsed;
  uint16_t parseIndex; // index to the next byte for parsing.
  uint16_t length;
  uint16_t messageCode; // "Message Code" for the corresponding Non TOM Command.
} sl_zigbee_af_gbz_message_parser_state_t;

typedef struct {
  uint8_t * payload;
  uint8_t payloadLength;
} sli_zigbee_af_gbz_payload_header;

/*
 * @brief A link list that keeps track of raw data that represents appended
 *        GBZ Use Case-Specific components.
 */
struct sli_zigbee_af_gbz_use_case_specific_component{
  uint8_t * payload;
  uint16_t payloadLength;
  struct sli_zigbee_af_gbz_use_case_specific_component * next;
};

typedef struct sli_zigbee_af_gbz_use_case_specific_component sli_zigbee_af_gbz_use_case_specific_component;

typedef struct {
  uint8_t * payload;
  uint16_t payloadLength;
  bool freeRequired;
} sl_zigbee_af_gbz_message_creator_result_t;

typedef struct {
  bool allocateMemoryForResponses;

  // used when allocateMemoryForResponses is false
  uint8_t * command;
  uint16_t commandIndex; // index to the next byte for appending.
  uint16_t commandLength;

  // used when allocateMemoryForResponses is true
  sli_zigbee_af_gbz_use_case_specific_component * responses;
  sli_zigbee_af_gbz_use_case_specific_component * lastResponse;
  sli_zigbee_af_gbz_payload_header * header;

  // otherwise.
  uint8_t nextEncryptedComponentZclSequenceNumber;
  uint8_t nextComponentZclSequenceNumber;
  uint8_t nextAdditionalHeaderFrameCounter;
  uint8_t * componentsCount;
  uint8_t * lastExtHeaderControlField;
  uint16_t messageCode;

  // assembled result
  sl_zigbee_af_gbz_message_creator_result_t result;
} sl_zigbee_af_gbz_message_creator_state_t;

/**
 * @name API
 * @{
 */

/**
 * @brief Assemble appended ZCL responses into 1 big chunk of memory.
 *
 */
sl_zigbee_af_gbz_message_creator_result_t * sl_zigbee_af_gbz_message_controller_creator_assemble(sl_zigbee_af_gbz_message_creator_state_t * state);

/**
 * @brief Get the default response byte from the GBZ ZCL Command.
 *
 * @param ZCL status.
 */
sl_zigbee_af_status_t sl_zigbee_af_gbz_message_controller_get_zcl_default_response(sl_zigbee_af_gbz_zcl_command_t * cmd);

/**
 * @brief Check the GBZ parser structure for any non-parsed commands.
 *
 * @param state A structure containing the bookkeeping information of parsed GBZ messages.
 */
bool sl_zigbee_af_gbz_message_controller_has_next_command(sl_zigbee_af_gbz_message_parser_state_t * state);

/**
 * @brief Clean any resources allocated during the parsing of GBZ message.
 *
 * @param state A pre-allocated structure that's updated to hold
 *   bookkeeping information of parsed GBZ messages.
 */
void sl_zigbee_af_gbz_message_controller_parser_cleanup(sl_zigbee_af_gbz_message_parser_state_t * state);

/**
 * @brief Initialize proper parsing states for decoding GBZ messages.
 *
 * The GBZ message payload and payload length are passed in as
 * arguments. Iterator functions, sl_zigbee_af_gbz_message_controller_has_next_command() and
 * sl_zigbee_af_gbz_message_controller_next_command() are used to iterate through
 * each of the embedded ZCL functions.
 *
 * @param state            A pre-allocated structure that's updated to hold
 *                         bookkeeping information for parsing GBZ messages.
 * @param gbzCommand       A pointer to GBZ messages.
 * @param gbzCommandLength Length of GBZ messages.
 * @param copyGbzCommand   A flag to indicate if the parser should be storing the GBZ
 *                         command locally for parsing.
 * @param messageCode      "Message Code" for the corresponding Non TOM command.
 *
 */
bool sl_zigbee_af_gbz_message_controller_parser_init(sl_zigbee_af_gbz_message_parser_state_t * state,
                                                     sl_zigbee_af_gbz_message_type_t type,
                                                     uint8_t * gbzCommand,
                                                     uint16_t gbzCommandLength,
                                                     bool copyGbzCommand,
                                                     uint16_t messageCode);

/**
 * @brief Append a ZCL command to a given GBZ creator structure.
 *
 *
 * @param state A pre-allocated structure that's updated to hold
 *   bookkeeping information of creating GBZ messages.
 * @param zclCmd A structure containing information for new ZCL command.
 *
 *  @return 0 - if the appending operation did not succeed
 *          else - number of appended bytes.
 */
uint16_t sl_zigbee_af_gbz_message_controller_append_command(sl_zigbee_af_gbz_message_creator_state_t * state, sl_zigbee_af_gbz_zcl_command_t * zclCmd);

/**
 * @brief Initialize proper states for construction of GBZ messages.
 *
 * Depending on the value of the argument (gbzCommand), the creator
 * behaves differently. If a NULL value is passed, the creator assumes you
 * want the API to allocate memory to store the appended responses.
 * Otherwise, the creator uses the provided buffer as the destination for
 * storing responses.
 *
 * Below is a general flow to make ZCL messages into a GBZ
 * message.
 *
 *  1. sl_zigbee_af_gbz_message_controller_creator_init() - create
 *  2. sl_zigbee_af_gbz_message_controller_append_command() - append
 *  3. sl_zigbee_af_gbz_message_controller_creator_assemble() - assemble result.
 *  4. sl_zigbee_af_gbz_message_controller_creator_cleanup() - memory clean up
 *
 * @param state      A pre-allocated structure that will be updated to hold
 *                   bookkeeping information for creating GBZ messages.
 * @param type       GBZ payload type: command, response, or alert.
 * @param alertCode  When type is alert this field contains the alert code.
 * @param timestamp  When type is alert this field contains the UTC when the alert occurred.
 * @param gbzCommand NULL - if the user wants API to allocate memory to store
 *                          responses.
 *                   Otherwise - a pointer to destination buffer for GBZ messages
 * @param gbzCommandLength Length of GBZ messages. this argument is
 *                         ignored if gbzCommand is NULL.
 *
 */
uint16_t sl_zigbee_af_gbz_message_controller_creator_init(sl_zigbee_af_gbz_message_creator_state_t * state,
                                                          sl_zigbee_af_gbz_message_type_t type,
                                                          uint16_t alertCode,
                                                          uint32_t timestamp,
                                                          uint16_t messageCode,
                                                          uint8_t * gbzCommand,
                                                          uint16_t gbzCommandLength);

/**
 * @brief Return the command size of the given parser structure.
 *
 * @param state A structure containing the bookkeeping information of parsed GBZ messages.
 */

uint8_t sl_zigbee_af_gbz_message_controller_get_component_size(sl_zigbee_af_gbz_message_parser_state_t * state);

/**
 * @brief Clean up/free all allocated memory used to
 *        store the overall GBZ response.
 */
void sl_zigbee_af_gbz_message_controller_creator_cleanup(sl_zigbee_af_gbz_message_creator_state_t * state);

/**
 * @brief Get the next available ZCL command from the given GBZ parser structure.
 *
 * If any payload is encrypted, the decrypted data will overwrite the old
 * data.
 *
 * @param state A structure that retains the bookkeeping info of parsing GBZ messages.
 * @param gbzZclCommand A pre-allocated buffer that will be modified with the
 *   next available ZCL command's information.
 */
void sl_zigbee_af_gbz_message_controller_next_command(sl_zigbee_af_gbz_message_parser_state_t * state, sl_zigbee_af_gbz_zcl_command_t * gbzZclCommand);

/**
 * @brief Print out all information retained in a sl_zigbee_af_gbz_zcl_command_t structure.
 */
void sl_zigbee_af_gbz_message_controller_print_command_info(sl_zigbee_af_gbz_zcl_command_t  * gbzZclCommand);

/**
 * @brief Indicate whether the ZCL payload will be encrypted.
 */
bool sl_zigbee_af_gbz_message_controller_get_encrypt_payload_flag(sl_zigbee_af_gbz_message_creator_state_t * state,
                                                                  sl_zigbee_af_gbz_zcl_command_t * resp);
/** @} */ // end of name API

/**
 * @name Callbacks
 * @{
 */

/**
 * @defgroup gbz_msg_controller_cb GBZ Message Controller
 * @ingroup af_callback
 * @brief Callbacks for GBZ Message Controller Component
 *
 */

/**
 * @addtogroup gbz_msg_controller_cb
 * @{
 */

/** @brief Decrypt data.
 *
 * This function is called by the GBZ Message Controller plugin to decrypt a
 * ZCL payload. If the decryption is successful, the application needs
 * to allocate memory for the new decrypted data and pass the pointer
 * via the "plainPayload" field as well as the plainPayloadLength field for the
 * length. The framework needs to free the allocated memory.
 * If the decryption fails, the callback return false and should not modify any
 * data.
 *
 * @param data   Ver.: always
 */
void sl_zigbee_af_gbz_message_controller_decrypt_data_cb(sl_zigbee_af_gbz_message_data_t *data);

/** @brief Encrypt data.
 *
 * This function is called by the GBZ Message Controller plugin to encrypt a
 * ZCL payload. If the encryption is successful, the application needs to
 * allocate memory for the new encrypted data and pass the pointer
 * via the "encryptedPayload" field as well as the encryptedPayloadLength field
 * for the length. The framework needs to free the allocated
 * memory. If the encryption fails, the callback return false and should not
 * modify any data.
 *
 * @param data   Ver.: always
 */
void sl_zigbee_af_gbz_message_controller_encrypt_data_cb(sl_zigbee_af_gbz_message_data_t *data);
/** @} */ // end of gbz_msg_controller_cb
/** @} */ // end of name Callbacks
/** @} */ // end of gbz-message-controller

#endif // #ifndef _GBZ_MESSAGE_CONTROLLER_
