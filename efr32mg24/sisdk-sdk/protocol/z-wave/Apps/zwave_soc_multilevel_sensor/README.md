# Multilevel Sensor

Shows the ability to advertise numerical sensor readings, such as temperature, and humidity. Multiple parameters can be set for the minimum and maximum values.

The Multilevel Sensor application is based on:

| <!-- -->                | <!-- -->                                  |
| :---------------------- | :---------------------------------------- |
| Role Type               | Reporting Sleeping End Device (RSS)       |
| Supporting Device Type  | Data reporting                            |
| Device Type             | Notification sensor                       |
| Generic Type            | Sensor Notification                       |
| Specific Type           | Notification Sensor                       |
| Requested security keys | S2\_UNAUTHENTICATED and S2\_AUTHENTICATED |

Multilevel Sensor transmits the following events:

- Environment monitoring
  - Temperature, and humidity measure

## Supported Command Classes

Multilevel Sensor implements mandatory and some optional command classes. The table below lists the supported Command Classes, their version, and their required Security class, if any.

| Command Class             | Version | Required Security Class        |
| :------------------------ | :-----: | :----------------------------- |
| Association               |    2    | Highest granted Security Class |
| Association Group Info    |    3    | Highest granted Security Class |
| Device Reset Locally      |    1    | Highest granted Security Class |
| Firmware Update Meta Data |    5    | Highest granted Security Class |
| Indicator                 |    3    | Highest granted Security Class |
| Manufacturer Specific     |    2    | Highest Granted Security Class |
| Multi-Channel Association |    3    | Highest granted Security Class |
| Powerlevel                |    1    | Highest granted Security Class |
| Security 2                |    1    | None                           |
| Supervision               |    1    | None                           |
| Transport Service         |    2    | None                           |
| Version                   |    3    | Highest granted Security Class |
| Wake Up                   |    2    | Highest granted Security Class |
| Z-Wave Plus Info          |    2    | None                           |
| Configuration             |    4    | Highest granted Security Class |
| Multilevel-Sensor         |   11    | Highest granted Security Class |

## Basic Command Class Mapping

Basic Command Class is not mapped to any of the supported command classes.

## Association Groups

The following table shows the available association groups.

<table>
<tr>
    <th>ID</th>
    <th>Name</th>
    <th>Node Count</th>
    <th>Description</th>
</tr><tr>
    <td>1</td>
    <td>Lifeline</td>
    <td>5</td>
    <td>
        <p>Supports the following command classes:</p>
        <ul>
            <li>Device Reset Locally: triggered upon reset.</li>
            <li>Indicator Report: Triggered when LED1 changes state.</li>
            <li>Configuring parameters: Minimum and maximum temperature levels can be set, errors can be detected if measured temperature is out of range</li>
            <li>Environmental measurements: Temperature and humidity values
can be read, triggered from other Z-Wave devices</li>
        </ul>
    </td>
</tr>
</table>

## Usage of Buttons and LED Status

The following LEDs and buttons shown in the next table below are used.

<table>
<tr>
    <th>Button</th>
    <th>Action</th>
    <th>Description</th>
</tr><tr>
    <td>RST</td>
    <td>Press</td>
    <td>
      Resets the firmware of an application (like losing power). All volatile memory will be cleared.<br>
    </td>
</tr><tr>
    <td>BTN0<sup>1</sup</td>
    <td>Press</td>
    <td>Sends Battery Report, temperature, and humidity data (only if the device is not sleeping)</td>
</tr><tr>
    <td rowspan="2">BTN1<sup>1</sup</td>
    <td>Press</td>
    <td>
        Enter "learn mode" (sending node info frame) to add/remove the device.<br>
        Removing the device from a network will reset it.
    </td>
</tr><tr>
    <td>Hold for at least 5 seconds and release</td>
    <td>Perform a reset to factory default operation of the device, and a Device Reset Locally Notification Command is sent via Lifeline.</td>
</tr>
</table>

<table>
<tr>
    <th>LED</th>
    <th>Description</th>
</tr><tr>
    <td>LED1</td>
    <td>
        Blinks with 1 Hz when learn mode is active.<br>
        Used for Indicator Command Class.
    </td>
</tr>
</table>

<sup>1</sup>: The application needs to be woken up for these operations.
It is not possible to wake up the BRD4400C and BRD4401C radio boards using BTN0
and BTN1, and the BRD2603A Thunderboard using BTN1.\
To support these functionalities, the affected buttons must be remapped to pins
that support waking up the chip from EM4 sleep mode.
Please refer to your hardware's documentation for further information.

## Firmware Update

This section will describe backward compatibility when upgrading the MultilevelSensor application from one SDK to a newer version. 

## CLI Support
In case CLI support is needed please install zw_cli_common component to the project. Please note the zw_cli_common component will modify the power consumption in case of sleeping applications. Like door lock keypad, sensor pir or multilevel sensor. CLI cannot work with sleep mode, after a reset the application stays awake until the user issues the enable_sleeping command. From that point CLI won't work  and sleep mode will be reached until the next reset.


<table>
<tr>
    <th>Command</th>
    <th>Arguments</th>
    <th>Description</th>
</tr>
<tr>
    <th>set_learn_mode</th>
    <td>It can be start or stop</td>
    <td>Starting or stopping the learn mode</td>
</tr>
<tr>
    <th>factory_reset</th>
    <td>-</td>
    <td>Executing factory reset</td>
</tr>
<tr>
    <th>get_dsk</th>
    <td>-</td>
    <td>Printing out the generated DSK of the device</td>
</tr>
<tr>
    <th>get_region</th>
    <td>-</td>
    <td>Printing out the set region of the application</td>
</tr>
<tr>
    <th>send_battery_and_sensor_report</th>
    <td>-</td>
    <td>Sending battery and sensor reports</td>
</tr>
<tr>
    <th>enable_sleeping</th>
    <td>-</td>
    <td>Lets the application go into sleep mode. After this command the CLI won't work until the next reset</td>
</tr>
</table>
