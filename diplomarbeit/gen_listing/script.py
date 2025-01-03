# this script was generated with ChatGPT see: TODO add link to docs of that

import os
import sys
import subprocess
import git
import json
from datetime import datetime
from pathlib import Path
from shutil import copyfile

def main(repo_path, config_path):
    try:
        # Load configuration from JSON
        with open(config_path, "r") as config_file:
            config = json.load(config_file)

        branch_name = config["branch"]
        start_commit = config["startCommit"]
        pdf_output_dir = config["pdf_output_dir"]
        html_index_file = config["html_index_file"]
        github_repo_url = config["githubRepoUrl"]

        # Ensure the output directory exists
        os.makedirs(pdf_output_dir, exist_ok=True)

        # Initialize the Git repository
        repo = git.Repo(repo_path)

        # Check if the branch exists
        if branch_name not in repo.heads:
            raise ValueError(f"Branch '{branch_name}' does not exist in the repository.")

        # Checkout the branch
        branch = repo.heads[branch_name]
        branch.checkout()

        # Start processing commits from the specified starting point
        commit_list = []
        found_start_commit = False
        last_pdf_hash = None

        for commit in repo.iter_commits(branch_name):
            if not found_start_commit:
                if commit.hexsha == start_commit:
                    found_start_commit = True
                else:
                    continue

            # Checkout the commit
            repo.git.checkout(commit.hexsha)

            # Run the predefined command
            pdf_filename = f"{commit.hexsha}.pdf"
            pdf_filepath = os.path.join(pdf_output_dir, pdf_filename)

            try:
                result = subprocess.run("htldoc build", check=True)
                copyfile("./build/out.pdf", pdf_filepath)
            except subprocess.CalledProcessError as e:
                print(f"Error generating PDF for commit {commit.hexsha}: {e}")
                continue

            # Check if the PDF content has changed
            if last_pdf_hash is not None:
                current_pdf_hash = get_file_hash(pdf_filepath)
                if current_pdf_hash == last_pdf_hash:
                    os.remove(pdf_filepath)  # Remove unchanged PDF
                    continue
                last_pdf_hash = current_pdf_hash
            else:
                last_pdf_hash = get_file_hash(pdf_filepath)

            # Store commit details
            commit_list.append({
                "hash": commit.hexsha,
                "date": commit.committed_datetime,
                "pdf": pdf_filename,
            })

        # Generate the HTML index
        generate_html_index(commit_list, pdf_output_dir, html_index_file, github_repo_url)
        print("Processing complete. HTML index generated.")

    except Exception as e:
        print(f"An error occurred: {e}")


def get_file_hash(filepath):
    import hashlib

    hasher = hashlib.sha256()
    with open(filepath, "rb") as f:
        buf = f.read()
        hasher.update(buf)
    return hasher.hexdigest()


def generate_html_index(commit_list, pdf_output_dir, html_index_file, github_repo_url):
    sorted_commits = sorted(commit_list, key=lambda x: x["date"])
    
    with open(html_index_file, "w") as html_file:
        html_file.write("<html><head><title>Commit PDFs</title></head><body>")
        html_file.write("<h1>Commit PDFs</h1>")
        html_file.write("<ul>")

        for commit in sorted_commits:
            commit_url = f"{github_repo_url}/commit/{commit['hash']}"
            html_file.write(
                f"<li><a href='{pdf_output_dir}/{commit['pdf']}'>{commit['hash']}</a> - "
                f"<a href='{commit_url}'>{commit['date'].strftime('%Y-%m-%d %H:%M:%S')}</a></li>"
            )

        html_file.write("</ul></body></html>")


if __name__ == "__main__":
    if len(sys.argv) != 3 or sys.argv[1] != "--config":
        print("Usage: python script.py --config <path_to_config_json>")
        sys.exit(1)

    config_path = sys.argv[2]

    if not os.path.isfile(config_path):
        print(f"Error: {config_path} is not a valid file.")
        sys.exit(1)

    repository_path = os.path.dirname(config_path)

    main(repository_path, config_path)
