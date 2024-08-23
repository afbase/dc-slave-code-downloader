#!/bin/bash

# Loop through all jpg files in the current directory
for file in *.jpg; do
    # Extract the number from the filename
    number=$(echo "$file" | sed 's/\.jpg$//')
    
    # Check if the extracted part is a valid number
    if [[ "$number" =~ ^[0-9]+$ ]]; then
        # Convert the number to a zero-padded three-digit format
        new_number=$(printf "%03d" "$number")
        
        # Only rename if the new name is different
        if [ "$file" != "${new_number}.jpg" ]; then
            git mv "$file" "${new_number}.jpg"
            echo "Renamed $file to ${new_number}.jpg"
        else
            echo "Skipped $file (already in correct format)"
        fi
    else
        echo "Skipped $file (not a number-based filename)"
    fi
done

echo "Renaming complete."
