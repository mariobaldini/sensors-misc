#!/bin/bash

SERIAL_NUMBER=`cat /proc/cpuinfo | grep Serial | awk -F ': ' '{print $2}'`
DATA=`sudo /home/pi/pcsensor.uncalibrated.arm.bin`

CMD="curl -X POST -d '$DATA' https://bus-map.herokuapp.com/sensor/$SERIAL_NUMBER"

echo "Running command: "$CMD
$CMD

