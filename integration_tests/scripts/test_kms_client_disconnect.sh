#!/bin/bash

# Function to get random number between 1 and 4
get_random() {
    echo $((RANDOM % 4 + 1))
}

# Function to execute command based on random number
execute_command() {
    case $1 in
        1)
            echo "=== Starting proxy at $(date '+%Y-%m-%d %H:%M:%S') ==="
            make run-proxy
            ;;
        2)
            echo "=== Stopping proxy at $(date '+%Y-%m-%d %H:%M:%S') ==="
            make stop-proxy
            ;;
        3)
            echo "=== Starting tester at $(date '+%Y-%m-%d %H:%M:%S') ==="
            make run-tester
            ;;
        4)
            echo "=== Stopping tester at $(date '+%Y-%m-%d %H:%M:%S') ==="
            make stop-tester
            ;;
    esac
}

# Main loop
echo "=== Starting random operations test at $(date '+%Y-%m-%d %H:%M:%S') ==="
echo "Press Ctrl+C to stop"

while true; do
    random_num=$(get_random)
    execute_command $random_num
    echo "=== Waiting 15 seconds before next operation ==="
    sleep 15
done 