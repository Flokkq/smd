#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#include "nec.h"

#if defined(_WIN32) || defined(_WIN64)

#include <windows.h>

#else
#include <unistd.h>
#endif

int parse_md(char *input, char *output_type, char *specific);

int check_requirements();

int validate_input(char *input, char *output_type, char *specific);

char *read_file(char *filename);

int md_to_html(char *filename, char *input);

int generate_css();

int inject_css(char *filename);

int handle_requirement_check(FILE *fp, char *cmd);

int parse_md(char *input, char *output_type, char *specific) {
    if (check_requirements() == 1) {
        noc_print(E, "Permission denied!\n");
        return 1;
    }

    if (validate_input(input, output_type, specific) == 1) {
        noc_print(E, "Invalid input");
        return 1;
    }

    noc_print(I, "Parsing markdown file...");
    char *md = read_file(input);
    if (md == NULL) {
        noc_print(E, "Could not read input not read file");
        free(md);
        return 1;
    }

    if (md_to_html(input, md) == 1) {
        noc_print(E, "Could not convert markdown to html");
        free(md);
        return 1;
    }

    if (generate_css() == 1) {
        noc_print(E, "Could not generate css");
        free(md);
        return 1;
    }

    if (inject_css(input) == 1) {
        noc_print(E, "Could not inject css");
        free(md);
        return 1;
    }

    return 0;
}

int check_requirements() {
    noc_print(I, "Checking requirements...");

    char *cmd = "npm ls -g";
    FILE *fp;

    int result = handle_requirement_check(fp, cmd);

    printf("Result: %d\n", result);
    if (result != 0) {
        noc_print(I, "Package 'github-markdown-css' is not installed globally. Installing...");

        #if defined(_WIN32) || defined(_WIN64)
            system("npm install -g github-markdown-css >> NULL 2>&1");
        #else
            system("sudo npm install -g github-markdown-css >> /dev/null 2>&1");
        #endif

        result = handle_requirement_check(fp, cmd);

        if (result != 0) {
            return 1;
        } else {
            noc_print(I, "'github-markdown-css' installed successfully.");
        }
    } else {
        noc_print(I, "'github-markdown-css' is already installed.");
    }
    return 0;
}


int handle_requirement_check(FILE *fp, char *cmd) {
    char buffer[1024];

#if defined(_WIN32) || defined(_WIN64)
    char fullCmd[1024];
    snprintf(fullCmd, sizeof(fullCmd), "%s | findstr github-markdown-css", cmd);
    fp = _popen(fullCmd, "r");
#else
    char fullCmd[1024];
    snprintf(fullCmd, sizeof(fullCmd), "%s | grep github-markdown-css", cmd);
    fp = popen(fullCmd, "r");
#endif

    if (fp == NULL) {
        noc_print(E, "Failed running commands to check requirements.");
        pclose(fp);
        exit(1);
    }

    return strcmp(fgets(buffer, sizeof(buffer), fp), "") == 0;
}

int validate_input(char *input, char *output, char *specific) {
    if (strcmp(input, "") == 0) {
        noc_print(E, "No input file specified.");
        return 1;
    }
    if (strcmp(output, "") == 0) {
        noc_print(E, "No output type specified.");
        return 1;
    }
    if (strcmp(specific, "") == 0) {
        noc_print(E, "No specific output type specified.");
        return 1;
    }
    return 0;
}

char *read_file(char *filename) {
    FILE *fp;
    char *buffer;
    size_t buffer_size = 2048;
    size_t total_size = 0;
    size_t bytes_read;

    printf("Reading file: %s\n", filename);
    fp = fopen(filename, "r");
    if (fp == NULL) {
        noc_print(E, "Could not open file %s.", filename);
        exit(1);
    }

    buffer = (char *) malloc(buffer_size);
    if (buffer == NULL) {
        noc_print(E, "Memory allocation failed for reading file %s.", filename);
        fclose(fp);
        exit(1);
    }

    while ((bytes_read = fread(buffer + total_size, 1, buffer_size - total_size, fp)) > 0) {
        total_size += bytes_read;
        char *new_buffer = realloc(buffer, total_size + buffer_size);
        if (new_buffer == NULL) {
            noc_print(E, "Memory reallocation failed for reading file %s.", filename);
            free(buffer);
            fclose(fp);
            exit(1);
        }
    }

    fclose(fp);

    buffer[total_size] = '\0';

    return buffer;
}

char *write_file(char *filename, char *content) {
    FILE *fp;
    fp = fopen(filename, "w");

    if (fp == NULL) {
        noc_print(E, "Could not open file %s.", filename);
        exit(1);
    }

    fprintf(fp, "%s", content);
}

int md_to_html(char *filename, char *input) {
    noc_print(I, "Creating html file...");

    int commandLength = 16384;
    char *command = malloc(commandLength);

    strtok(filename, ".");

    if (command == NULL) {
        noc_print(E, "Could not allocate memory.");
        return 1;
    }

    char *parsed_filename = malloc(strlen(filename) + 5);
    if (parsed_filename == NULL) {
        noc_print(E, "Could not allocate memory.");
        return 1;
    }

    snprintf(parsed_filename, strlen(filename) + 32, "%s_pre.html", filename);


#if defined(_WIN32) || defined(_WIN64)
    snprintf(command, commandLength,
             "gh api `"
             "  --method POST `"
             "  -H \"Accept: application/vnd.github+json\" `"
             "  -H \"X-GitHub-Api-Version: 2022-11-28\" `"
             "  /markdown `"
             "  -f text=\"%s\" `"
             "  -f mode=\"gfm\" | Out-File -FilePath \"%s\"",
             input, abs_path);
#else
    snprintf(command, commandLength,
        "gh api \\\n"
        "  --method POST \\\n"
        "  -H \"Accept: application/vnd.github+json\" \\\n"
        "  -H \"X-GitHub-Api-Version: 2022-11-28\" \\\n"
        "  /markdown \\\n"
        "  -f text='%s' \\\n"
        "  -f mode='gfm' > %s",
        input, parsed_filename);

#endif

    printf("Command: %s\n", command);

    system(command);
    free(command);
    return 0;
}

int inject_css(char *filename) {
    noc_print(I, "Injecting css...");

    char *html_file_name = malloc(strlen(filename) + 32);
    if (html_file_name == NULL) {
        noc_print(E, "Could not allocate memory.");
        return 1;
    }

    snprintf(html_file_name, strlen(filename) + 32, "%s_pre.html", filename);

    char *html_file = read_file(html_file_name);
    if (html_file == NULL) {
        noc_print(E, "Could not read html file");
        return 1;
    }

    printf("%s\n", html_file);

    char *html_structure = malloc(strlen(html_file) + 1024);
    if (html_structure == NULL) {
        noc_print(E, "Could not allocate memory.");
        return 1;
    }

    snprintf(html_structure, strlen(html_file) + 1024,
             "<!DOCTYPE html>\n"
             "<html lang=\"en\">\n"
             "<head>\n"
             "    <meta charset=\"UTF-8\">\n"
             "    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n"
             "    <title>%s</title>\n"
             "    <link rel=\"stylesheet\" href=\"github-markdown-css\">\n"
             "</head>\n"
             "<body>\n"
             "    <article class=\"markdown-body\">\n"
             "        %s\n"
             "    </article>\n"
             "</body>\n"
             "</html>", filename, html_file);

    snprintf(html_file_name, strlen(filename) + 32, "%s.html", filename);

    char *cmd = malloc(strlen(html_file_name) + 32);

    #if defined(_WIN32) || defined(_WIN64)
        snprintf(cmd, strlen(html_file_name) + 32, "del %s_pre.html", filename);
        system(cmd);
    #else
        snprintf(cmd, strlen(html_file_name) + 32, "rm %s_pre.html", filename);
        system(cmd);
    #endif

    write_file(html_file_name, html_structure);

    return 0;
}

int generate_css() {
    noc_print(I, "Generating css file...");
}