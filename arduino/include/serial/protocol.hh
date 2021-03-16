#if !defined(_SERIAL_PROTOCOL_HH)
#define _SERIAL_PROTOCOL_HH

#include <stdint.h>

#define PROTOCOL_VERSION 0x01

namespace proto
{

    enum status
    {
        OK = 0x00,
        FRAME_STORAGE_ERROR = 0x01
    };

    enum packet
    {
        HOST_HANDSHAKE = 0x01,
        COPROCESSOR_HANDSHAKE = 0x02,
        ENTER_PROGRAMMING_MODE = 0x03,
        STATUS_UPDATE = 0x04,
        FRAME_UPLOAD = 0x05,
        ANIMATION_VERIFY = 0x06,
        PLAYBACK_REQUEST = 0x07,
        STOP_REQUEST = 0x08,
        PLAYBACK_ENDED = 0x09
    };

    namespace hostbound
    {
        void send_handshake_response(uint8_t maximum_frame_count, uint8_t display_width, uint8_t display_height);
        void send_status(status status);
        void send_upload_verification(uint8_t parity, uint8_t frame_count);
        void send_playback_end_notification();
    } // namespace hostbound

} // namespace proto

#endif // _SERIAL_PROTOCOL_HH
