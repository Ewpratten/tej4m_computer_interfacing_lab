#include <Arduino.h>

#include "interface.hh"
#include "protocol.hh"

void interface::write_state_update(uint8_t x, uint8_t y)
{
    Serial.write(proto::PacketType::STATE_UPDATE);
    Serial.write(0x02);
    Serial.write(x);
    Serial.write(y);
    Serial.write('\r');
    Serial.write('\n');
}