import os
import csv

# Pre-processes the data set and splits it into 10k record chunks.
# Pre-processing selects fields that are relevant to the creation of embeddings.

def if_exists(word, output):
    return output if word else ""

def with_article(word):
    lower_word = word.lower()
    return f"an {word}" if lower_word[0] in ["a", "e", "i", "o", "u"] else f"a {word}"

done_title_row = False
cols = {}
records = []

# Get the directory path of the current script.
script_dir = os.path.dirname(os.path.abspath(__file__))

print("Running dataset through pre-processor...")

# Read CSV file and process (around half a million!) records.
csv_file = os.path.join(script_dir, "../openaccess/MetObjects.csv")
with open(csv_file, newline="", encoding="utf-8") as csvfile:
    reader = csv.reader(csvfile)
    for idx, record in enumerate(reader):
        if not done_title_row:
            cols = {val: idx for idx, val in enumerate(record)}
            done_title_row = True
            records.append(["id", "is_highlight", "description"])
            continue

        # Build description line.
        desc_parts = [
            f"{record[cols['Title']]}",
            f"Type: {record[cols['Object Name']]}",
            if_exists(record[cols["Culture"]], f"Culture: {record[cols['Culture']]}"),
            if_exists(
                record[cols["Country"]], f"Country of origin: {record[cols['Country']]}"
            ),
            if_exists(
                record[cols["Artist Display Name"]],
                f"Artist: {record[cols['Artist Display Name']]}",
            ),
            if_exists(record[cols["Medium"]], f"Medium: {record[cols['Medium']]}"),
            if_exists(
                record[cols["Object End Date"]],
                f"Date: {record[cols['Object End Date']]}",
            ),
            if_exists(
                record[cols["Credit Line"]], f"Credit: {record[cols['Credit Line']]}"
            ),
            ". A highlight of the Met's collection."
            if record[cols["Is Highlight"]] == "True"
            else "",
        ]
        desc_line = (
            "; ".join(filter(None, desc_parts)).replace("\n", "").replace("\r", "")
        )

        # Create new record.
        new_rec = [
            record[cols["Object ID"]],
            1 if record[cols["Is Highlight"]] == "True" else 0,
            desc_line,
        ]
        records.append(new_rec)

# Output directories relative to script directory.
output_dir = os.path.join(script_dir, "../data")
chunked_dir = os.path.join(output_dir, "chunked")

print("Writing pre-processed records to data/met_object_descriptions.csv")
os.makedirs(output_dir, exist_ok=True)

output_csv_file = os.path.join(output_dir, "met_object_descriptions.csv")
with open(output_csv_file, "w", newline="", encoding="utf-8") as f:
    writer = csv.writer(f)
    writer.writerows(records)

print("Saving 10k record chunks in data/chunked")
os.makedirs(chunked_dir, exist_ok=True)

# Remove header row.
columns = records.pop(0)

# Split the description records into 10k chunks.
for i in range(1, int(len(records) / 10000) + 1):
    chunk_file = f"met_object_descriptions_{(i-1)*10}-{i*10}k.csv"
    chunk_csv_file = os.path.join(chunked_dir, chunk_file)
    with open(chunk_csv_file, "w", newline="", encoding="utf-8") as f:
        writer = csv.writer(f)
        writer.writerows([columns] + records[(i - 1) * 10000 : i * 10000])

print("💅 Done!")
