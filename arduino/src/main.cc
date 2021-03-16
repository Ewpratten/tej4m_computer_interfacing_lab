#include <Arduino.h>
#include <serial/protocol.hh>

void setup()
{

    // Init the serial connection
    Serial.begin(9600);

    // Init IO

    // Wait for a valid host connection
    while (true)
    {

        // Try to complete the handshake with the host
        if(Serial.available()){
            
        }
    }
}

void loop()
{
}