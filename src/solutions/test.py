def count_unique_antinodes(map_data):
    # Parse the map into a dictionary of positions by frequency
    antennas = {}
    for y, row in enumerate(map_data):
        for x, char in enumerate(row):
            if char != '.':
                if char not in antennas:
                    antennas[char] = []
                antennas[char].append((x, y))
    
    antinodes = set()  # Set to store unique antinode positions
    max_x = len(map_data[0])
    max_y = len(map_data)
    
    # For each frequency, calculate antinodes
    for freq, positions in antennas.items():
        n = len(positions)
        for i in range(n):
            for j in range(i + 1, n):
                x1, y1 = positions[i]
                x2, y2 = positions[j]
                
                # Check for horizontal alignment
                if y1 == y2:
                    dist = abs(x2 - x1)
                    if dist % 3 == 0:  # Valid distance for antinodes
                        d = dist // 3
                        mid_x = (x1 + x2) // 2
                        antinodes.add((x1 + d, y1))
                        antinodes.add((x2 - d, y1))
                
                # Check for vertical alignment
                elif x1 == x2:
                    dist = abs(y2 - y1)
                    if dist % 3 == 0:  # Valid distance for antinodes
                        d = dist // 3
                        mid_y = (y1 + y2) // 2
                        antinodes.add((x1, y1 + d))
                        antinodes.add((x1, y2 - d))
    
    # Filter antinodes within bounds of the map
    valid_antinodes = {
        (x, y) for x, y in antinodes if 0 <= x < max_x and 0 <= y < max_y
    }
    
    return len(valid_antinodes)

# Example input map
map_data = [
    "............",
    "........0...",
    ".....0......",
    ".......0....",
    "....0.......",
    "......A.....",
    "............",
    "............",
    "........A...",
    ".........A..",
    "............",
    "............"
]

# Calculate and print the number of unique antinodes
result = count_unique_antinodes(map_data)
print("Number of unique antinodes:", result)
