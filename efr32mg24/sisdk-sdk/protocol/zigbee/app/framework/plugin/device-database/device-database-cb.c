/***************************************************************************//**
 * @file
 * @brief
 *******************************************************************************
 * # License
 * <b>Copyright 2021 Silicon Laboratories Inc. www.silabs.com</b>
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

/** @brief Discovery Complete
 *
 * This function is called when a device in the database has been set to
 * SL_ZIGBEE_AF_DEVICE_DISCOVERY_STATUS_DONE.
 *
 * @param device A pointer to the information struct about the device.
 * Ver.: always
 */
WEAK(void sl_zigbee_af_device_database_discovery_complete_cb(const sl_zigbee_af_device_info_t*device))
{
}
