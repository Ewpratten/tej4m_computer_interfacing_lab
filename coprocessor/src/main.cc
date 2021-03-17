#include <Arduino.h>
#include <stdint.h>

#include "interface.hh"

// I/O Pins
#define X_INPUT_PIN A0
#define Y_INPUT_PIN A1

// Timeout on heartbeats
#define HEARBEAT_TIMEOUT_MS 500

// Update threshold
#define DEADZONE 20

// State tracking
uint8_t last_x, last_y;
uint64_t last_packet_timestamp;

void setup()
{

    // Configure I/O
    pinMode(X_INPUT_PIN, INPUT);
    pinMode(Y_INPUT_PIN, INPUT);

    // Configure serial
    Serial.begin(9600);

    // Wait for a valid connection
    while (!Serial.availableForWrite())
    {
    }
}

void loop()
{

    // Get system time
    uint64_t timestamp = millis();

    // Get the joystick state
    uint8_t current_x = (analogRead(X_INPUT_PIN) / 1023.0) * 255;
    uint8_t current_y = (analogRead(Y_INPUT_PIN) / 1023.0) * 255;

    // Check if a new packet needs to be sent
    bool deadzone_update = (abs(current_x - last_x) > DEADZONE) || (abs(current_y - last_y) > DEADZONE);
    bool heartbeat_update = timestamp - last_packet_timestamp > HEARBEAT_TIMEOUT_MS;

    // Handle sending packet if needed
    if (deadzone_update || heartbeat_update)
    {
        interface::write_state_update(current_x, current_y);
        last_packet_timestamp = timestamp;
        last_x = current_x;
        last_y = current_y;
    }
}