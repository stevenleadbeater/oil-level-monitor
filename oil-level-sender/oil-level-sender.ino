#include <EtherCard.h>

// ethernet interface mac address, must be unique on the LAN
byte mymac[] = { 0x74,0x69,0x69,0x2D,0x30,0x31 };
const int pingPin = 4; // Trigger Pin of Ultrasonic Sensor
const int echoPin = 5; // Echo Pin of Ultrasonic Sensor

byte Ethernet::buffer[700];
uint32_t timer;
Stash stash;

void setup () {
  Serial.begin(57600);
  Serial.println("\n[webClient]");

  if (ether.begin(sizeof Ethernet::buffer, mymac, 8) == 0) {
    Serial.println( "Failed to access Ethernet controller");
  }
  
  ether.hisport = 8120;//to access  local host
  ether.hisip[0] = 10;
  ether.hisip[1] = 10;
  ether.hisip[2] = 6;
  ether.hisip[3] = 254;
  
  Serial.println(F("Setting up DHCP"));
  if (!ether.dhcpSetup())
    Serial.println(F("DHCP failed"));

  ether.printIp("My IP: ", ether.myip);
  ether.printIp("Netmask: ", ether.netmask);
  ether.printIp("GW IP: ", ether.gwip);
  ether.printIp("DNS IP: ", ether.dnsip);
    
}

void loop () {
  ether.packetLoop(ether.packetReceive());

  if (millis() > timer) {
    Serial.println("\n>>> REQ");
    timer = millis() + 10000;

    long distance = readDistance();

    byte sd = stash.create();
    stash.print("{");
    stash.print("\"id\":1,");
    stash.print("\"distance\":");
    stash.print(distance);
    stash.print("}");
    stash.save();

    // generate the header with payload - note that the stash size is used,
    // and that a "stash descriptor" is passed in as argument using "$H"
    Stash::prepare(PSTR("POST / HTTP/1.0" "\r\n"
                "Content-Length: $D" "\r\n"
                "Content-Type: application/json" "\r\n"
                "\r\n"
                "$H"),
    stash.size(), sd);

    // send the packet - this also releases all stash buffers once done
    ether.tcpSend();
  }
}

long readDistance() {
    long duration, cm;
    pinMode(pingPin, OUTPUT);
    digitalWrite(pingPin, LOW);
    delayMicroseconds(2);
    digitalWrite(pingPin, HIGH);
    delayMicroseconds(10);
    digitalWrite(pingPin, LOW);
    pinMode(echoPin, INPUT);
    duration = pulseIn(echoPin, HIGH);
    cm = microsecondsToCentimeters(duration);
    return cm;
}

long microsecondsToInches(long microseconds) {
   return microseconds / 74 / 2;
}

long microsecondsToCentimeters(long microseconds) {
   return microseconds / 29 / 2;
}
