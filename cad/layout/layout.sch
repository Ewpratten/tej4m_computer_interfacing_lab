EESchema Schematic File Version 4
EELAYER 30 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 1 1
Title ""
Date ""
Rev ""
Comp ""
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
$Comp
L Display_Character:MAN71A U?
U 1 1 6052AE5F
P 6600 1850
F 0 "U?" H 6600 2517 50  0000 C CNN
F 1 "MAN71A" H 6600 2426 50  0000 C CNN
F 2 "Display_7Segment:MAN71A" H 6100 1150 50  0001 L CNN
F 3 "https://www.digchip.com/datasheets/parts/datasheet/161/MAN3640A-pdf.php" H 6610 2190 50  0001 L CNN
	1    6600 1850
	1    0    0    -1  
$EndComp
$Comp
L MCU_Microchip_ATmega:ATmega328P-AU U?
U 1 1 60527514
P 2450 2200
F 0 "U?" H 2450 611 50  0000 C CNN
F 1 "ATmega328P-AU" H 2450 520 50  0000 C CNN
F 2 "Package_QFP:TQFP-32_7x7mm_P0.8mm" H 2450 2200 50  0001 C CIN
F 3 "http://ww1.microchip.com/downloads/en/DeviceDoc/ATmega328_P%20AVR%20MCU%20with%20picoPower%20Technology%20Data%20Sheet%2040001984A.pdf" H 2450 2200 50  0001 C CNN
	1    2450 2200
	1    0    0    -1  
$EndComp
$Comp
L 74xx:74HC595 U?
U 1 1 60528BAB
P 4750 1850
F 0 "U?" H 4750 2631 50  0000 C CNN
F 1 "74HC595" H 4750 2540 50  0000 C CNN
F 2 "" H 4750 1850 50  0001 C CNN
F 3 "http://www.ti.com/lit/ds/symlink/sn74hc595.pdf" H 4750 1850 50  0001 C CNN
	1    4750 1850
	1    0    0    -1  
$EndComp
Wire Wire Line
	6300 1450 5750 1450
Wire Wire Line
	5750 1450 5750 2150
Wire Wire Line
	5750 2150 5150 2150
Wire Wire Line
	6300 1550 5800 1550
Wire Wire Line
	5800 1550 5800 2050
Wire Wire Line
	5800 2050 5150 2050
Wire Wire Line
	6300 1650 5850 1650
Wire Wire Line
	5850 1650 5850 1950
Wire Wire Line
	5850 1950 5150 1950
Wire Wire Line
	6300 1750 5900 1750
Wire Wire Line
	5900 1750 5900 1850
Wire Wire Line
	5900 1850 5150 1850
Wire Wire Line
	6300 1850 5950 1850
Wire Wire Line
	5950 1850 5950 1800
Wire Wire Line
	5950 1800 5700 1800
Wire Wire Line
	5700 1800 5700 1750
Wire Wire Line
	5700 1750 5150 1750
Wire Wire Line
	6300 1950 6000 1950
Wire Wire Line
	6000 1950 6000 1700
Wire Wire Line
	6000 1700 5650 1700
Wire Wire Line
	5650 1700 5650 1650
Wire Wire Line
	5650 1650 5150 1650
Wire Wire Line
	6300 2050 6050 2050
Wire Wire Line
	6050 2050 6050 1600
Wire Wire Line
	6050 1600 5600 1600
Wire Wire Line
	5600 1600 5600 1550
Wire Wire Line
	5600 1550 5150 1550
Wire Wire Line
	6300 2250 6100 2250
Wire Wire Line
	6100 2250 6100 1500
Wire Wire Line
	6100 1500 5550 1500
Wire Wire Line
	5550 1500 5550 1450
Wire Wire Line
	5550 1450 5150 1450
Wire Wire Line
	6900 2250 6900 2650
Wire Wire Line
	4750 2650 4750 2550
Wire Wire Line
	6900 2650 4750 2650
Wire Wire Line
	4750 2650 4750 3700
Wire Wire Line
	4750 3700 3550 3700
Connection ~ 4750 2650
Wire Wire Line
	2450 700  2450 600 
Wire Wire Line
	2450 600  3800 600 
Wire Wire Line
	4750 600  4750 1250
Wire Wire Line
	4350 1750 4250 1750
Wire Wire Line
	4250 1750 4250 600 
Connection ~ 4250 600 
Wire Wire Line
	4250 600  4750 600 
Wire Wire Line
	4350 2050 4350 2650
Wire Wire Line
	4350 2650 4750 2650
Wire Wire Line
	4350 1950 4150 1950
Wire Wire Line
	4150 1950 4150 1400
Wire Wire Line
	4150 1400 3050 1400
Wire Wire Line
	4350 1450 4350 1300
Wire Wire Line
	4350 1300 3050 1300
Wire Wire Line
	4350 1650 4050 1650
Wire Wire Line
	4050 1650 4050 1200
Wire Wire Line
	4050 1200 3050 1200
$Comp
L Switch:SW_MEC_5E SW?
U 1 1 605737FE
P 3800 3100
F 0 "SW?" H 3800 3485 50  0000 C CNN
F 1 "SW_MEC_5E" H 3800 3394 50  0000 C CNN
F 2 "" H 3800 3400 50  0001 C CNN
F 3 "http://www.apem.com/int/index.php?controller=attachment&id_attachment=1371" H 3800 3400 50  0001 C CNN
	1    3800 3100
	1    0    0    -1  
$EndComp
Wire Wire Line
	3600 3000 3550 3000
Wire Wire Line
	3500 3000 3500 1000
Wire Wire Line
	3500 1000 3050 1000
Wire Wire Line
	4000 3000 4150 3000
Wire Wire Line
	4150 3000 4150 2200
Wire Wire Line
	4150 2200 3800 2200
Wire Wire Line
	3800 2200 3800 600 
Connection ~ 3800 600 
Wire Wire Line
	3800 600  4250 600 
$Comp
L Device:R 100ohm
U 1 1 60582E2A
P 3550 3400
F 0 "100ohm" H 3620 3446 50  0000 L CNN
F 1 "R" H 3620 3355 50  0000 L CNN
F 2 "" V 3480 3400 50  0001 C CNN
F 3 "~" H 3550 3400 50  0001 C CNN
	1    3550 3400
	1    0    0    -1  
$EndComp
Wire Wire Line
	3550 3250 3550 3000
Connection ~ 3550 3000
Wire Wire Line
	3550 3000 3500 3000
Wire Wire Line
	3550 3550 3550 3700
Connection ~ 3550 3700
Wire Wire Line
	3550 3700 2450 3700
$EndSCHEMATC
