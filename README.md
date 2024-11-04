# Input Timing

This project is to:

- capture and comparing timing data from various systems
  - game engine frame time
  - keyboard input time
  - midi instrument input time
  - audio "heard" time
- reconcile them for accuracy

The general issues that I need to tackle are:

- audio lag - sound is heard slightly after it is scheduled
- input is coerced to frame time (want to understand actual offset)

The subtle timing difference matter because low milliseconds (10-15m feels delayed, 20-30ms is perceptibly separate).
