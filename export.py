import re
import shlex
import shutil
import sqlite3
import os

def save_blob_to_file(blob_data, file_path):
    # Ensure the directory exists
    os.makedirs(os.path.dirname(file_path), exist_ok=True)
    
    # Write the BLOB data to the file
    with open(file_path, 'wb') as file:
        file.write(blob_data)

# Connect to the SQLite database
conn = sqlite3.connect('db.sqlite')
cursor = conn.cursor()

# Execute a query to retrieve BLOB data
cursor.execute("SELECT title, image FROM cards where length(image) > 4000 limit 1000")

for row in cursor.fetchall():
    title, image = row

    # Escape spaces in the title
    safe_title = shlex.quote(title)

    safe_title = re.sub(r'[^a-zA-Z0-9]', '_', safe_title)

    # Create the file path with subfolder
    file_path = os.path.join('output_directory', safe_title, f'file.jpg')
    
    # Save the BLOB to a file
    save_blob_to_file(image, file_path)
    
    print(f"Saved BLOB with title {title} to {file_path}")

# Close the database connection
conn.close()
