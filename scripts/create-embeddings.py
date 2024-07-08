import os
from transformers import AutoTokenizer, AutoModel
from sklearn.decomposition import PCA
import torch
import numpy as np
import json
import pandas as pd
import torch.nn.functional as F
import time
import glob

model_name = "sentence-transformers/all-MiniLM-L12-v2"

desired_embedding_dimensions = 5

# Get the directory path of the current script.
script_dir = os.path.dirname(os.path.abspath(__file__))

# The dataset is split into 10K record chunks using preprocess.py,
# e.g., data/chunked/met_object_descriptions_0-10k.csv
# This is to avoid memory issues when processing the entire dataset
# on low-powered devices.
file_paths = os.path.join(script_dir, "../data/chunked/*.csv")

output_dir_embeddings = os.path.join(script_dir, "../data/embeddings")

# Load pre-trained model and tokenizer.
tokenizer = AutoTokenizer.from_pretrained(model_name)
model = AutoModel.from_pretrained(model_name)

# Check if GPU is available.
if torch.cuda.is_available():
    print("GPU is available. Using GPU 👩‍🎤")
    device = torch.device("cuda")
else:
    print("GPU not available. Using CPU 👵")
    device = torch.device("cpu")

# Mean pooling for feature extraction.
# https://paperswithcode.com/method/average-pooling
def mean_pooling(model_output, attention_mask):
    token_embeddings = model_output[0]
    input_mask_expanded = (
        attention_mask.unsqueeze(-1).expand(token_embeddings.size()).float()
    )
    return torch.sum(token_embeddings * input_mask_expanded, 1) / torch.clamp(
        input_mask_expanded.sum(1), min=1e-9
    )


# Get embeddings and normalize them.
def get_embeddings(text):
    inputs = tokenizer(
        text, return_tensors="pt", truncation=True, padding=True, max_length=512
    )
    with torch.no_grad():
        outputs = model(**inputs)
    sentence_embeddings = mean_pooling(outputs, inputs["attention_mask"])
    sentence_embeddings = F.normalize(sentence_embeddings, p=2, dim=1)
    return sentence_embeddings.squeeze().numpy()


print("\n🧬 Computing embeddings for dataset:")
os.makedirs(output_dir_embeddings, exist_ok=True)

# Process each chunk file in file_paths.
for dataset_file_name in glob.glob(file_paths):
    # Compute output file name.
    base_name = os.path.basename(dataset_file_name).replace(".csv", "")
    output_file_name = f"{output_dir_embeddings}/vec_{base_name}.json"

    print(f"\nProcessing {dataset_file_name}...")

    # Skip if output file already exists.
    if os.path.exists(output_file_name):
        print(f"\t✋ {output_file_name}: already exists.")
        continue

    print(f"\n\t⏳ Processing {dataset_file_name}...")
    start_time = time.time()

    # Load the CSV file.
    df = pd.read_csv(dataset_file_name)

    # Convert the DataFrame to a list of dictionaries.
    art_objects = df.to_dict(orient="records")

    # Generate embeddings for descriptions.
    vectors = np.array([get_embeddings(obj["description"]) for obj in art_objects])

    # Apply PCA to reduce dimensionality of embeddings.
    pca = PCA(n_components=desired_embedding_dimensions)
    vectors = pca.fit_transform(vectors)

    # Initialize a list to hold the combined data.
    combined_data = []
    for i, obj in enumerate(art_objects):
        combined_entry = [obj["id"], vectors[i].tolist()]
        combined_data.append(combined_entry)

    # Store embeddings data with clusters.
    with open(output_file_name, "w") as f:
        json.dump(combined_data, f)

    print(
        f"\t🤙 Computed {model_name} embeddings: {time.time() - start_time:.2f} seconds"
    )

print("\n📦 Combining results:")

combined_data = []
combined_data_filename = f"{output_dir_embeddings}/combined.json"

# Combine the vectors from all the JSON files.
if os.path.exists(combined_data_filename):
    print(f"\t✋ Combined data file ({combined_data_filename}) already exists.")
    with open(combined_data_filename, "r") as f:
        combined_data = json.load(f)
else:
    for dataset_file_name in glob.glob(file_paths):
        start_time = time.time()

        # Compute input file name.
        base_name = os.path.basename(dataset_file_name).replace(".csv", "")
        input_file_name = f"{output_dir_embeddings}/vec_{base_name}.json"

        # Read the JSON file and add its contents to an array.
        with open(input_file_name, "r") as f:
            data = json.load(f)
            combined_data.extend(data)

        print(f"\t⏳ Processed {input_file_name} with {len(data)} entries.")

    # Save the vectors  to a JSON file.
    with open(combined_data_filename, "w") as f:
        json.dump(combined_data, f)

    print(
        f"\n\t🤙 Saved combined data file ({combined_data_filename}). Total entries: {len(combined_data)}"
    )

print("\nDone! 🎉")
