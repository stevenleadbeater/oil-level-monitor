# Arduino Sketch for Monitoring Oil Tank Level

## Components Required
  
  You will need the following components −
  
      10 × Bits of wire
      1 × Arduino Nano
      1 × ULTRASONIC Sensor (HC-SR04)
      1 x HiLetgo ENC28J60 ENC28J60-I/SO HR911105A
      1 x (Optional) Active POE Splitter with Micro USB Female to Mini USB Male Adapter

### HC-SR04

The HC-SR04 ultrasonic sensor uses SONAR to determine the distance of an object just like the bats do. It offers excellent non-contact range detection with high accuracy and stable readings in an easy-to-use package from 2 cm to 400 cm or 1” to 13 feet.

The operation is not affected by sunlight or black material, although acoustically, soft materials like cloth can be difficult to detect. It comes complete with ultrasonic transmitter and receiver module.

![Picture of HC-SR04](https://github.com/stevenleadbeater/oil-level-monitor/raw/master/oil-level-sender/ultrasonic_sensor.jpg.png)

#### PIN Connections (Using Arduino UNO or Arduino NANO):

| HC-SR04  | Arduino Nano | Notes                                             |
|----------|--------------|---------------------------------------------------|
| VCC      | 3.3V         |                                                   |
| GND      | GND          |                                                   |
| TRIG     | Pin 4        |                                                   |
| ECHO     | Pin 5        |                                                   |

### ENC28J60

The Ethernet Controller (ENC28J60) is a so called SPI device and uses the SPI pins (8, 11, 12, 13) of your Arduino.

SS stands for Slave Select, used to enable or disable the slave device (the Ethernet module in this case).
MOSI stands for Master Output Slave Input, or in other words: Arduino OUTPUT (data from Arduino to Ethernet Controller).
MISO stands for the opposite, Master Input Slave Output, or: Arduino INPUT (data from Ethernet Controller to Arduino).
SCK is the clock used for SPI timing.

The [EtherCard](https://github.com/njh/ethercard) library is used for TCP/IP communications

![Picture of ENC28J60](https://github.com/stevenleadbeater/oil-level-monitor/raw/master/oil-level-sender/ENC28J60.jpeg)

#### PIN Connections (Using Arduino UNO or Arduino NANO):

| ENC28J60 | Arduino Nano | Notes                                             |
|----------|--------------|---------------------------------------------------|
| VCC      | 3.3V         |                                                   |
| GND      | GND          |                                                   |
| SCK      | Pin 13       |                                                   |
| MISO     | Pin 12       |                                                   |
| MOSI     | Pin 11       |                                                   |
| CS       | Pin 8        |Selectable with the ether.begin() function:        |
|          |              |`ether.begin(sizeof Ethernet::buffer, mymac, 8)`   |