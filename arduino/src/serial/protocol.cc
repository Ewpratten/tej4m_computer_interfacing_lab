#include <serial/protocol.hh>

#include <Arduino.h>

inline void write_packet_size(uint16_t size)
{
    Serial.write((uint8_t)size >> 8);
    Serial.write((uint8_t)size);
}

void proto::hostbound::send_handshake_response(uint8_t maximum_frame_count, uint8_t display_width, uint8_t display_height)
{
    Serial.write(proto::packet::COPROCESSOR_HANDSHAKE);
    write_packet_size(0x03);
    Serial.write(maximum_frame_count);
    Serial.write(display_width);
    Serial.write(display_height);
}

void proto::hostbound::send_status(proto::status status)
{
    Serial.write(proto::packet::STATUS_UPDATE);
    write_packet_size(0x01);
    Serial.write(status);
}

void proto::hostbound::send_upload_verification(uint8_t parity, uint8_t frame_count)
{
    Serial.write(proto::packet::ANIMATION_VERIFY);
    write_packet_size(0x02);
    Serial.write(parity);
    Serial.write(frame_count);
}

void proto::hostbound::send_playback_end_notification()
{
    Serial.write(proto::packet::PLAYBACK_ENDED);
    write_packet_size(0x00);
}
