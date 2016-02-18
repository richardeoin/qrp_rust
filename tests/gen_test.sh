#!/bin/bash
#
# Generates batches of random test vectors using wsprcode, jt4code, jt9code and jt65code utilities
#
#

echo "Attempting to generate wspr test vectors..."
echo

# Test vectors for wspr
#
#

WSPR_CODE=${WSPR_PATH}wsprcode

echo -en "Looking for wsprcode utility... "

if [[ -x "$WSPR_CODE" ]]
then
    echo "Found wsprcode"
    echo
else
    echo "Not found."
    echo
    echo "You'll need to download and build wspr. You can find details at http://physics.princeton.edu/pulsar/k1jt/devel.html"
    echo "When I wrote this, the following guide from George Smart M1GEO was useful: https://www.george-smart.co.uk/wiki/Compiling_WSPR"
    echo
    echo "Make sure the wsprcode utility appears in your path, or set the WSPR_PATH enviroment variable"
    exit
fi


WSPR_COUNT=100
WSPR_FILE=testvectors.wspr

echo "==========================================================================" > $WSPR_FILE
for i in `seq 1 $WSPR_COUNT`;
do
    # Callsign
    C1=$(head -c5 /dev/urandom | tr -cd '0-9A-Z' | head -c1) # alpha / digit or space
    C2=$(head -c1000 /dev/urandom | tr -cd '0-9A-Z' | head -c1) # alpha / digit
    C3=$(head -c5000 /dev/urandom | tr -cd '0-9' | head -c1) # digit
    C4=$(head -c35 /dev/urandom | tr -cd 'A-Z' | head -c3) # random 0-3 alpha
    CALLSIGN=$C1$C2$C3$C4

    # Locator
    L1=$(head -c5000 /dev/urandom | tr -cd 'A-R' | head -c2)
    L2=$(head -c2000 /dev/urandom | tr -cd '0-9' | head -c2)
    LOCATOR=$L1$L2

    # Power
    POWER=$(shuf -e 0 3 7 10 13 17 20 23 27 30 33 37 40 43 47 50 53 57 60 -n 1)      # valid WSPR powers


    $WSPR_CODE "$CALLSIGN $LOCATOR $POWER" >> $WSPR_FILE
    echo "==========================================================================" >> $WSPR_FILE


    perc=$(( i * 100 / WSPR_COUNT ))
    num=$(( i * 76 / WSPR_COUNT ))
    bar=
    if [ $num -gt 0 ]; then
        bar=$(printf "%0.s-" $(seq 1 $num))
    fi
    # Print the progress bar.
    line=$(printf "%s [%-${width}s] (%d%%)" "WSPR" "$bar" "$perc")
    echo -en "${line}\r"
done

echo "WSPR: wrote $WSPR_COUNT test vectors to $WSPR_FILE"

echo
echo
echo

# Test vectors for jt4, jt9 and jt65
#
#

echo "Attempting to generate wsjt-x test vectors..."
echo

# Paths for wsjt-x utilites
JT4_CODE=${WSJTX_PATH}jt4code
JT9_CODE=${WSJTX_PATH}jt9code
JT65_CODE=${WSJTX_PATH}jt65code

echo -en "Looking for wsjt-x utilites..."

if [[ -x "$JT4_CODE" ]] && [[ -x "$JT9_CODE" ]] && [[ -x "$JT65_CODE" ]]
then
    echo "Found jt4code, jt9code and jt65code"
    echo
else
    echo "Not Found."
    echo
    echo "You'll need to download and build wsjt-x. You can find details at http://physics.princeton.edu/pulsar/k1jt/devel.html"
    echo
    echo "Make sure the jt4code, jt9code and jt65code utilities appear in your path, or set the WSJTX_PATH enviroment variable"
    exit
fi

#
# jt4
#

JT4_COUNT=100
JT4_FILE=testvectors.jt4

echo "==========================================================================" > $JT4_FILE
for i in `seq 1 $JT4_COUNT`;
do
    $JT4_CODE "$(head -c100 /dev/urandom | tr -cd '0-9A-Z +./?-' | head -c13)" >> $JT4_FILE
    echo "==========================================================================" >> $JT4_FILE


    perc=$(( i * 100 / JT4_COUNT ))
    num=$(( i * 76 / JT4_COUNT ))
    bar=
    if [ $num -gt 0 ]; then
        bar=$(printf "%0.s-" $(seq 1 $num))
    fi
    # Print the progress bar.
    line=$(printf "%s [%-${width}s] (%d%%)" "JT4" "$bar" "$perc")
    echo -en "${line}\r"
done

echo "JT4: wrote $JT4_COUNT test vectors to $JT4_FILE"


#
# jt9
#

JT9_COUNT=100
JT9_FILE=testvectors.jt9

echo "==========================================================================" > $JT9_FILE
for i in `seq 1 $JT9_COUNT`;
do
    $JT9_CODE "$(head -c100 /dev/urandom | tr -cd '0-9A-Z +./?-' | head -c13)" >> $JT9_FILE
    echo "==========================================================================" >> $JT9_FILE


    perc=$(( i * 100 / JT9_COUNT ))
    num=$(( i * 76 / JT9_COUNT ))
    bar=
    if [ $num -gt 0 ]; then
        bar=$(printf "%0.s-" $(seq 1 $num))
    fi
    # Print the progress bar.
    line=$(printf "%s [%-${width}s] (%d%%)" "JT9" "$bar" "$perc")
    echo -en "${line}\r"
done

echo "JT9: wrote $JT9_COUNT test vectors to $JT9_FILE"


#
# jt65
#

JT65_COUNT=100
JT65_FILE=testvectors.jt65

echo "==========================================================================" > $JT65_FILE
for i in `seq 1 $JT65_COUNT`;
do
    $JT65_CODE "$(head -c100 /dev/urandom | tr -cd '0-9A-Z +./?-' | head -c13)" >> $JT65_FILE
    echo "==========================================================================" >> $JT65_FILE


    perc=$(( i * 100 / JT65_COUNT ))
    num=$(( i * 76 / JT65_COUNT ))
    bar=
    if [ $num -gt 0 ]; then
        bar=$(printf "%0.s-" $(seq 1 $num))
    fi
    # Print the progress bar.
    line=$(printf "%s [%-${width}s] (%d%%)" "JT65" "$bar" "$perc")
    echo -en "${line}\r"
done

echo "JT65: wrote $JT65_COUNT test vectors to $JT65_FILE"
