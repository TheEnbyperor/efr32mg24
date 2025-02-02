/***************************************************************************//**
 * @file
 * @brief Implemented routines for storing future prepayment modes.
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
#include "prepayment-server.h"
#include "prepayment-modes-table.h"
#include "prepayment-tick.h"

#include "prepayment-server-config.h"

#define CHANGE_PAYMENT_MODE_DATETIME_NOW    0x00000000
#define CHANGE_PAYMENT_MODE_DATETIME_CANCEL 0xFFFFFFFF
#define INVALID_PAYMENT_CONTROL_CONFIG      0xFFFF
#define INVALID_UTC_TIME  0xFFFFFFFF
#define END_TIME_NEVER    0xFFFFFFFF
#define MAX_NUM_PENDING_PAYMENT_MODES   SL_ZIGBEE_AF_PLUGIN_PREPAYMENT_SERVER_MAX_PENDING_PAYMENT_MODES

typedef struct {
  uint32_t providerId;
  uint32_t issuerEventId;
  uint32_t startTimeUtc;
  uint32_t endTimeUtc;
  uint16_t paymentControlConfig;
  uint8_t  endpoint;
  uint8_t  active;
} sli_zigbee_af_pending_payment_mode;

static sli_zigbee_af_pending_payment_mode PendingPaymentModes[MAX_NUM_PENDING_PAYMENT_MODES];

static void initPrepaymentModesTableIndex(uint8_t x);
void updatePaymentControlConfiguration(uint8_t endpoint, uint16_t paymentControlConfig);
uint16_t sli_zigbee_af_prepayment_modes_get_current_mode(void);
void scheduleNextMode(uint8_t endpoint);

void sli_zigbee_af_init_prepayment_modes_table()
{
  uint8_t x;
  for ( x = 0; x < MAX_NUM_PENDING_PAYMENT_MODES; x++ ) {
    initPrepaymentModesTableIndex(x);
  }
}

static void initPrepaymentModesTableIndex(uint8_t x)
{
  if ( x < MAX_NUM_PENDING_PAYMENT_MODES ) {
    PendingPaymentModes[x].providerId = 0;
    PendingPaymentModes[x].issuerEventId = 0;
    PendingPaymentModes[x].startTimeUtc = INVALID_UTC_TIME;
    PendingPaymentModes[x].endTimeUtc = INVALID_UTC_TIME;
    PendingPaymentModes[x].paymentControlConfig = INVALID_PAYMENT_CONTROL_CONFIG;
    PendingPaymentModes[x].endpoint = 0;
    PendingPaymentModes[x].active = 0;
  }
}

void sl_zigbee_af_prepayment_schedule_prepayment_mode(uint8_t endpoint, uint32_t providerId, uint32_t issuerEventId, uint32_t startTimeUtc, uint16_t paymentControlConfig)
{
  uint8_t  x;
  uint8_t  oldestIndex = 0xFF;
  uint32_t oldestEventId = 0xFFFFFFFF;
  uint32_t endTimeUtc = END_TIME_NEVER;

  if ( startTimeUtc == CHANGE_PAYMENT_MODE_DATETIME_NOW ) {
    startTimeUtc = sl_zigbee_af_get_current_time();
  }

  // Adjust overlapping start/end times with this event based on event IDs.
  // Also, find an index to store the event - the first unused.
  for ( x = 0; x < MAX_NUM_PENDING_PAYMENT_MODES; x++ ) {
    if ( startTimeUtc == CHANGE_PAYMENT_MODE_DATETIME_CANCEL ) {
      // CANCEL operation - look for matching provider ID & event ID.
      if ( (providerId == PendingPaymentModes[x].providerId) && (issuerEventId == PendingPaymentModes[x].issuerEventId) ) {
        // Found a match that should be cancelled.
        sl_zigbee_af_prepayment_cluster_println("Cancelling scheduled prepayment mode.");
        initPrepaymentModesTableIndex(x);
        break;
      }
    } else {
      if ( (PendingPaymentModes[x].startTimeUtc == INVALID_UTC_TIME) && (oldestEventId != 0) ) {
        // Unused index
        oldestIndex = x;
        oldestEventId = 0;
      } else {
        // Update END time on any overlapping events with smaller event IDs to avoid overlapping.
        // Also search for an index to add the new event.
        if ( (issuerEventId > PendingPaymentModes[x].issuerEventId)
             && (startTimeUtc < PendingPaymentModes[x].endTimeUtc) ) {
          PendingPaymentModes[x].endTimeUtc = startTimeUtc;
        } else if ( (PendingPaymentModes[x].issuerEventId > issuerEventId)
                    && (PendingPaymentModes[x].startTimeUtc > startTimeUtc) ) {
          // Found payment mode with later start time, and larger event ID.
          endTimeUtc = PendingPaymentModes[x].startTimeUtc;
        }
        //else if( PendingPaymentModes[x].issuerEventId < oldestEventId ){
        // We could possibly store a new event over the oldest event ID, but this seems bad.
        //oldestIndex = x;
        //oldestEventId = PendingPaymentModes[x].issuerEventId;
        //}
      }
    }
  }
  if ( oldestIndex < MAX_NUM_PENDING_PAYMENT_MODES ) {
    x = oldestIndex;
    PendingPaymentModes[x].endpoint = endpoint;
    PendingPaymentModes[x].providerId = providerId;
    PendingPaymentModes[x].issuerEventId = issuerEventId;
    PendingPaymentModes[x].startTimeUtc = startTimeUtc;
    PendingPaymentModes[x].endTimeUtc = endTimeUtc;
    PendingPaymentModes[x].paymentControlConfig = paymentControlConfig;
    sl_zigbee_af_prepayment_cluster_println("Adding prepayment mode, x=%d", x);
  }
  //scheduleNextMode( endpoint );
  sl_zigbee_af_prepayment_cluster_schedule_tick_cb(endpoint, PREPAYMENT_TICK_CHANGE_PAYMENT_MODE_EVENT);
}

void updatePaymentControlConfiguration(uint8_t endpoint, uint16_t paymentControlConfig)
{
  (void) sl_zigbee_af_write_attribute(endpoint, ZCL_PREPAYMENT_CLUSTER_ID,
                                      ZCL_PAYMENT_CONTROL_CONFIGURATION_ATTRIBUTE_ID, CLUSTER_MASK_SERVER,
                                      (uint8_t *)&paymentControlConfig, ZCL_BITMAP16_ATTRIBUTE_TYPE);
}

uint32_t sl_zigbee_af_prepayment_server_seconds_until_payment_mode_event(uint32_t timeNowUtc)
{
  uint32_t minDelaySec = EVENT_TIME_NO_PENDING_EVENTS;
  uint8_t x;
  uint32_t minNextTimeUtc = INVALID_UTC_TIME;
  uint32_t nextTimeUtc;

  for ( x = 0; x < MAX_NUM_PENDING_PAYMENT_MODES; x++ ) {
    nextTimeUtc = INVALID_UTC_TIME;

    if ( PendingPaymentModes[x].endTimeUtc <= timeNowUtc ) {
      // expire this entry
      initPrepaymentModesTableIndex(x);
    } else if ( PendingPaymentModes[x].active ) {
      // This event is current - next event when event expires
      //minDelaySec = PendingPaymentModes[i].endTimeUtc;
      nextTimeUtc = PendingPaymentModes[x].endTimeUtc;
    } else if ( (PendingPaymentModes[x].startTimeUtc <= timeNowUtc)
                && (PendingPaymentModes[x].endTimeUtc >= timeNowUtc) ) {
      // Found an entry that is not active that should be running now.
      // NOTE:  Earlier 'else if' checks to see if entry is already active.
      // Mark it as active and set nextTimeUtc to NOW.
      // We can abort the for loop immediately since we have an event that needs to run NOW.
      PendingPaymentModes[x].active = true;
      minDelaySec = 0;
      break;
    } else if ( PendingPaymentModes[x].startTimeUtc != INVALID_UTC_TIME ) {
      nextTimeUtc = PendingPaymentModes[x].startTimeUtc;
    }

    if ( nextTimeUtc < minNextTimeUtc ) {
      minNextTimeUtc = nextTimeUtc;
    }
  }
  if ( minNextTimeUtc != INVALID_UTC_TIME ) {
    if ( minNextTimeUtc > timeNowUtc ) {
      minDelaySec = minNextTimeUtc - timeNowUtc;
    } else {
      minDelaySec = 0;
    }
  }
  return minDelaySec;
}

void sl_zigbee_af_prepayment_server_set_payment_mode(uint8_t endpoint)
{
  uint16_t paymentControlConfig = sli_zigbee_af_prepayment_modes_get_current_mode();
  if ( paymentControlConfig != INVALID_PAYMENT_CONTROL_CONFIG ) {
    // Eventually, this will be used to change prepayment modes, etc.
    // For now, it exists to make sure I understand how to invoke it.
    updatePaymentControlConfiguration(endpoint, paymentControlConfig);
  }
}

uint16_t sli_zigbee_af_prepayment_modes_get_current_mode()
{
  uint8_t x;
  uint32_t now;
  uint32_t largestEventId = 0;
  uint8_t largestEventIdIndex = 0xFF;
  uint16_t currentPaymentControlConfig = INVALID_PAYMENT_CONTROL_CONFIG;

  now = sl_zigbee_af_get_current_time();

  // Invalidate all old entries and get the most recent paymentControlConfiguration.
  for ( x = 0; x < MAX_NUM_PENDING_PAYMENT_MODES; x++ ) {
    if ( PendingPaymentModes[x].endTimeUtc <= now ) {
      initPrepaymentModesTableIndex(x);
    }
    if ( (PendingPaymentModes[x].startTimeUtc <= now)
         && (PendingPaymentModes[x].endTimeUtc > now) ) {
      if ( PendingPaymentModes[x].issuerEventId > largestEventId ) {
        largestEventId = PendingPaymentModes[x].issuerEventId;
        largestEventIdIndex = x;
      }
    }
  }
  if ( largestEventIdIndex < MAX_NUM_PENDING_PAYMENT_MODES ) {
    x = largestEventIdIndex;
    currentPaymentControlConfig = PendingPaymentModes[x].paymentControlConfig;
    PendingPaymentModes[x].active = true;
  }
  return currentPaymentControlConfig;
}
