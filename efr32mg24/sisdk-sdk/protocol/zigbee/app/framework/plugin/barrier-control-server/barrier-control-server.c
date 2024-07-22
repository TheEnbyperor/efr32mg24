/***************************************************************************//**
 * @file
 * @brief Implementation for the Barrier Control Server plugin.
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
#include "app/framework/plugin/barrier-control-server/barrier-control-server.h"
#ifdef SL_COMPONENT_CATALOG_PRESENT
#include "sl_component_catalog.h"
#endif
#ifdef SL_CATALOG_ZIGBEE_ZCL_FRAMEWORK_CORE_PRESENT
#include "zap-cluster-command-parser.h"
#endif

// We need this for initializating default reporting configurations.
#include "app/framework/plugin/reporting/reporting.h"

typedef struct {
  uint8_t currentPosition;
  uint8_t targetPosition;
  bool increasing;
  uint32_t delayMs;
} State;
static State state;

#ifdef SL_ZIGBEE_SCRIPTED_TEST
  #define ZCL_USING_BARRIER_CONTROL_CLUSTER_OPEN_PERIOD_ATTRIBUTE
  #define ZCL_USING_BARRIER_CONTROL_CLUSTER_CLOSE_PERIOD_ATTRIBUTE
  #define ZCL_USING_BARRIER_CONTROL_CLUSTER_BARRIER_OPEN_EVENTS_ATTRIBUTE
  #define ZCL_USING_BARRIER_CONTROL_CLUSTER_BARRIER_CLOSE_EVENTS_ATTRIBUTE
  #define ZCL_USING_BARRIER_CONTROL_CLUSTER_BARRIER_COMMAND_OPEN_EVENTS_ATTRIBUTE
  #define ZCL_USING_BARRIER_CONTROL_CLUSTER_BARRIER_COMMAND_CLOSE_EVENTS_ATTRIBUTE
#endif

// -----------------------------------------------------------------------------
// Framework initialization

// -----------------------------------------------------------------------------
// Accessing attributes

uint8_t sli_zigbee_af_barrier_control_server_get_barrier_position(uint8_t endpoint)
{
  uint8_t position;
  sl_zigbee_af_status_t status = sl_zigbee_af_read_server_attribute(endpoint,
                                                                    ZCL_BARRIER_CONTROL_CLUSTER_ID,
                                                                    ZCL_BARRIER_POSITION_ATTRIBUTE_ID,
                                                                    &position,
                                                                    sizeof(position));
  assert(status == SL_ZIGBEE_ZCL_STATUS_SUCCESS);
  return position;
}

void sli_zigbee_af_barrier_control_server_set_barrier_position(uint8_t endpoint,
                                                               uint8_t position)
{
  sl_zigbee_af_status_t status
    = sl_zigbee_af_write_server_attribute(endpoint,
                                          ZCL_BARRIER_CONTROL_CLUSTER_ID,
                                          ZCL_BARRIER_POSITION_ATTRIBUTE_ID,
                                          &position,
                                          ZCL_INT8U_ATTRIBUTE_TYPE);
  assert(status == SL_ZIGBEE_ZCL_STATUS_SUCCESS);
}

bool sli_zigbee_af_barrier_control_server_is_partial_barrier_supported(uint8_t endpoint)
{
  uint8_t bitmap;
  sl_zigbee_af_status_t status
    = sl_zigbee_af_read_server_attribute(endpoint,
                                         ZCL_BARRIER_CONTROL_CLUSTER_ID,
                                         ZCL_BARRIER_CAPABILITIES_ATTRIBUTE_ID,
                                         &bitmap,
                                         sizeof(bitmap));
  assert(status == SL_ZIGBEE_ZCL_STATUS_SUCCESS);
  return READBITS(bitmap, SL_ZIGBEE_AF_BARRIER_CONTROL_CAPABILITIES_PARTIAL_BARRIER);
}

static uint16_t getOpenOrClosePeriod(uint8_t endpoint, bool open)
{
  uint16_t period = 0;
  sl_zigbee_af_attribute_id_t attributeId = 0xFFFF;
#if defined(ZCL_USING_BARRIER_CONTROL_CLUSTER_BARRIER_OPEN_PERIOD_ATTRIBUTE)
  if (open) {
    attributeId = ZCL_BARRIER_OPEN_PERIOD_ATTRIBUTE_ID;
  }
#endif
#if defined(ZCL_USING_BARRIER_CONTROL_CLUSTER_BARRIER_CLOSE_PERIOD_ATTRIBUTE)
  if (!open) {
    attributeId = ZCL_BARRIER_CLOSE_PERIOD_ATTRIBUTE_ID;
  }
#endif

  if (attributeId != 0xFFFF) {
    sl_zigbee_af_status_t status
      = sl_zigbee_af_read_server_attribute(endpoint,
                                           ZCL_BARRIER_CONTROL_CLUSTER_ID,
                                           attributeId,
                                           (uint8_t *)&period,
                                           sizeof(period));
    assert(status == SL_ZIGBEE_ZCL_STATUS_SUCCESS);
  }

  return period;
}

static void setMovingState(uint8_t endpoint, uint8_t state)
{
  sl_zigbee_af_status_t status
    = sl_zigbee_af_write_server_attribute(endpoint,
                                          ZCL_BARRIER_CONTROL_CLUSTER_ID,
                                          ZCL_BARRIER_MOVING_STATE_ATTRIBUTE_ID,
                                          &state,
                                          ZCL_ENUM8_ATTRIBUTE_TYPE);
  assert(status == SL_ZIGBEE_ZCL_STATUS_SUCCESS);
}

uint16_t sli_zigbee_af_barrier_control_server_get_safety_status(uint8_t endpoint)
{
  uint16_t safetyStatus;
  sl_zigbee_af_status_t status
    = sl_zigbee_af_read_server_attribute(endpoint,
                                         ZCL_BARRIER_CONTROL_CLUSTER_ID,
                                         ZCL_BARRIER_SAFETY_STATUS_ATTRIBUTE_ID,
                                         (uint8_t *)&safetyStatus,
                                         sizeof(safetyStatus));
  assert(status == SL_ZIGBEE_ZCL_STATUS_SUCCESS);
  return safetyStatus;
}

static bool isRemoteLockoutOn(uint8_t endpoint)
{
  uint16_t safetyStatus
    = sli_zigbee_af_barrier_control_server_get_safety_status(endpoint);
  return READBITS(safetyStatus,
                  SL_ZIGBEE_AF_BARRIER_CONTROL_SAFETY_STATUS_REMOTE_LOCKOUT);
}

void sli_zigbee_af_barrier_control_server_increment_events(uint8_t endpoint,
                                                           bool open,
                                                           bool command)
{
  uint8_t mask = (0
#if defined(ZCL_USING_BARRIER_CONTROL_CLUSTER_BARRIER_OPEN_EVENTS_ATTRIBUTE)
                  | (open && !command
                     ? BIT(0)
                     : 0)
#endif
#if defined(ZCL_USING_BARRIER_CONTROL_CLUSTER_BARRIER_CLOSE_EVENTS_ATTRIBUTE)
                  | (!open && !command
                     ? BIT(1)
                     : 0)
#endif
#if defined(ZCL_USING_BARRIER_CONTROL_CLUSTER_BARRIER_COMMAND_OPEN_EVENTS_ATTRIBUTE)
                  | (open && command
                     ? BIT(2)
                     : 0)
#endif
#if defined(ZCL_USING_BARRIER_CONTROL_CLUSTER_BARRIER_COMMAND_CLOSE_EVENTS_ATTRIBUTE)
                  | (!open && command
                     ? BIT(3)
                     : 0)
#endif
                  );

  sl_zigbee_af_attribute_id_t baseEventAttributeId = ZCL_BARRIER_OPEN_EVENTS_ATTRIBUTE_ID;
  for (size_t bit = 0; bit < 4; bit++) {
    if (READBIT(mask, bit)) {
      sl_zigbee_af_attribute_id_t attributeId = baseEventAttributeId + bit;
      uint16_t events;
      sl_zigbee_af_status_t status
        = sl_zigbee_af_read_server_attribute(endpoint,
                                             ZCL_BARRIER_CONTROL_CLUSTER_ID,
                                             attributeId,
                                             (uint8_t *)&events,
                                             sizeof(events));
      assert(status == SL_ZIGBEE_ZCL_STATUS_SUCCESS);

      // Section 7.1.2.1.5-8 says that this events counter SHALL NOT roll over.
      // The maximum 16-bit unsigned integer in Zigbee is 0xFFFE, so we have this
      // check here.
      if (events != UINT16_MAX - 1) {
        events++;
        status = sl_zigbee_af_write_server_attribute(endpoint,
                                                     ZCL_BARRIER_CONTROL_CLUSTER_ID,
                                                     attributeId,
                                                     (uint8_t *)&events,
                                                     ZCL_INT16U_ATTRIBUTE_TYPE);
        assert(status == SL_ZIGBEE_ZCL_STATUS_SUCCESS);
      }
    }
  }
}

// -----------------------------------------------------------------------------
// Opening/closing barrier

static uint8_t getCurrentPosition(uint8_t endpoint)
{
  // If the BarrierPosition attribute does not store the exact position of the
  // barrier, then it will be set to 0xFF. If this is the case, then we have no
  // way of knowing the position of the barrier. Let's guess that the barrier is
  // open so that we don't leave the barrier open when it should be closed.
  uint8_t currentPositionFromAttribute
    = sli_zigbee_af_barrier_control_server_get_barrier_position(endpoint);
  return ((currentPositionFromAttribute
           == SL_ZIGBEE_ZCL_BARRIER_CONTROL_BARRIER_POSITION_UNKNOWN)
          ? SL_ZIGBEE_ZCL_BARRIER_CONTROL_BARRIER_POSITION_OPEN
          : currentPositionFromAttribute);
}

static uint32_t calculateDelayMs(uint8_t endpoint,
                                 uint8_t targetPosition,
                                 bool *opening)
{
  uint8_t currentPosition
    = sli_zigbee_af_barrier_control_server_get_barrier_position(endpoint);
  *opening = targetPosition > currentPosition;
  uint8_t positionDelta = (*opening
                           ? targetPosition - currentPosition
                           : currentPosition - targetPosition);
  uint16_t openOrClosePeriodDs = getOpenOrClosePeriod(endpoint, *opening);
  uint32_t openOrClosePeriodMs
    = openOrClosePeriodDs * MILLISECOND_TICKS_PER_DECISECOND;

  // We use a minimum delay so that our barrier changes position in a realistic
  // amount of time.
  if (openOrClosePeriodDs == 0 || positionDelta == 0) {
    return MIN_POSITION_CHANGE_DELAY_MS;
  } else {
    uint32_t delayMs = openOrClosePeriodMs / positionDelta;
    return (delayMs < MIN_POSITION_CHANGE_DELAY_MS
            ? MIN_POSITION_CHANGE_DELAY_MS
            : delayMs);
  }
}

void sl_zigbee_af_barrier_control_cluster_server_tick_cb(uint8_t endpoint)
{
  if (state.currentPosition == state.targetPosition) {
    sli_zigbee_af_barrier_control_server_set_barrier_position(endpoint, state.currentPosition);
    setMovingState(endpoint, SL_ZIGBEE_ZCL_BARRIER_CONTROL_MOVING_STATE_STOPPED);
    sl_zigbee_zcl_deactivate_server_tick(endpoint, ZCL_BARRIER_CONTROL_CLUSTER_ID);
  } else {
    if (state.increasing) {
      if (++state.currentPosition == 1) {
        // Zero -> nonzero: open event
        sli_zigbee_af_barrier_control_server_increment_events(endpoint, true, false);
      }
    } else {
      if (--state.currentPosition == 0) {
        // Nonzero -> zero: close event
        sli_zigbee_af_barrier_control_server_increment_events(endpoint, false, false);
      }
    }
    sli_zigbee_af_barrier_control_server_set_barrier_position(endpoint,
                                                              (sli_zigbee_af_barrier_control_server_is_partial_barrier_supported(endpoint)
                                                               ? state.currentPosition
                                                               : SL_ZIGBEE_ZCL_BARRIER_CONTROL_BARRIER_POSITION_UNKNOWN));
    setMovingState(endpoint,
                   (state.increasing
                    ? SL_ZIGBEE_ZCL_BARRIER_CONTROL_MOVING_STATE_OPENING
                    : SL_ZIGBEE_ZCL_BARRIER_CONTROL_MOVING_STATE_CLOSING));
    sl_zigbee_zcl_schedule_server_tick(endpoint, ZCL_BARRIER_CONTROL_CLUSTER_ID, state.delayMs);
  }
}

// -----------------------------------------------------------------------------
// Handling commands

static void sendDefaultResponse(sl_zigbee_af_status_t status)
{
  if (sl_zigbee_af_send_immediate_default_response(status) != SL_STATUS_OK) {
    sl_zigbee_af_barrier_control_cluster_println("Failed to send default response");
  }
}

bool sl_zigbee_af_barrier_control_cluster_barrier_control_go_to_percent_cb(sl_zigbee_af_cluster_command_t *cmd)
{
  sl_zcl_barrier_control_cluster_barrier_control_go_to_percent_command_t cmd_data;
  uint8_t endpoint = sl_zigbee_af_current_command()->apsFrame->destinationEndpoint;
  sl_zigbee_af_status_t status = SL_ZIGBEE_ZCL_STATUS_SUCCESS;

  if (zcl_decode_barrier_control_cluster_barrier_control_go_to_percent_command(cmd, &cmd_data)
      != SL_ZIGBEE_ZCL_STATUS_SUCCESS) {
    return false;
  }

  sl_zigbee_af_barrier_control_cluster_println("RX: GoToPercentCallback p=%d", cmd_data.percentOpen);

  if (isRemoteLockoutOn(endpoint)) {
    status = SL_ZIGBEE_ZCL_STATUS_FAILURE;
  } else if (cmd_data.percentOpen > 100 // "100" means "100%", so greater than that is invalid
             || (!sli_zigbee_af_barrier_control_server_is_partial_barrier_supported(endpoint)
                 && cmd_data.percentOpen != SL_ZIGBEE_ZCL_BARRIER_CONTROL_BARRIER_POSITION_CLOSED
                 && cmd_data.percentOpen != SL_ZIGBEE_ZCL_BARRIER_CONTROL_BARRIER_POSITION_OPEN)) {
    status = SL_ZIGBEE_ZCL_STATUS_INVALID_VALUE;
  } else {
    state.currentPosition = getCurrentPosition(endpoint);
    state.targetPosition = cmd_data.percentOpen;
    state.delayMs = calculateDelayMs(endpoint,
                                     state.targetPosition,
                                     &state.increasing);
    sl_zigbee_af_barrier_control_cluster_println("Scheduling barrier move from %d to %d with %dms delay",
                                                 state.currentPosition,
                                                 state.targetPosition,
                                                 state.delayMs);
    sl_zigbee_zcl_schedule_server_tick(endpoint, ZCL_BARRIER_CONTROL_CLUSTER_ID, state.delayMs);

    if (state.currentPosition < state.targetPosition) {
      sli_zigbee_af_barrier_control_server_increment_events(endpoint, true, true);
    } else if (state.currentPosition > state.targetPosition) {
      sli_zigbee_af_barrier_control_server_increment_events(endpoint, false, true);
    }
  }

  sendDefaultResponse(status);

  return true;
}

bool sl_zigbee_af_barrier_control_cluster_barrier_control_stop_cb(void)
{
  uint8_t endpoint = sl_zigbee_af_current_command()->apsFrame->destinationEndpoint;
  sl_zigbee_zcl_deactivate_server_tick(endpoint, ZCL_BARRIER_CONTROL_CLUSTER_ID);
  setMovingState(endpoint, SL_ZIGBEE_ZCL_BARRIER_CONTROL_MOVING_STATE_STOPPED);
  sendDefaultResponse(SL_ZIGBEE_ZCL_STATUS_SUCCESS);
  return true;
}

uint32_t sl_zigbee_af_barrier_control_cluster_server_command_parse(sl_service_opcode_t opcode,
                                                                   sl_service_function_context_t *context)
{
  (void)opcode;

  sl_zigbee_af_cluster_command_t *cmd = (sl_zigbee_af_cluster_command_t *)context->data;
  bool wasHandled = false;

  if (!cmd->mfgSpecific) {
    switch (cmd->commandId) {
      case ZCL_BARRIER_CONTROL_GO_TO_PERCENT_COMMAND_ID:
      {
        wasHandled = sl_zigbee_af_barrier_control_cluster_barrier_control_go_to_percent_cb(cmd);
        break;
      }
      case ZCL_BARRIER_CONTROL_STOP_COMMAND_ID:
      {
        wasHandled = sl_zigbee_af_barrier_control_cluster_barrier_control_stop_cb();
        break;
      }
    }
  }

  return ((wasHandled)
          ? SL_ZIGBEE_ZCL_STATUS_SUCCESS
          : SL_ZIGBEE_ZCL_STATUS_UNSUP_COMMAND);
}
