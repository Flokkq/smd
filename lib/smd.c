#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#include "nec.h"

#if defined(_WIN32) || defined(_WIN64)

#include <windows.h>

#else
#include <unistd.h>
#endif

int set_md_flavour(char* flavour);

int parse_md(char *input, char *output_type, char *specific);

int check_requirements();

int validate_input(char *input, char *output_type, char *specific);

int md_to_html(char *filename, char *input);

int generate_css();

int inject_css(char *filename);

int handle_requirement_check(FILE *fp, char *cmd);


int set_md_flavour(char *flavour) {
    char *flavours[] = {"dark", "light", "auto"};

    for (size_t i = 0; i < sizeof(flavours) / sizeof(flavours[0]); i++) {
        if (strcmp(flavour, flavours[i]) == 0) {
            nec_print(NEC_INFO, "Setting flavour to %s", flavour);
            nec_write_file("res\\md_flavour", flavour);
            return 0;
        }
    }

    int i_flavour = strtol(flavour, NULL, 10);
    if (i_flavour < 1 || i_flavour > 3) {
        nec_print(NEC_ERROR, "Invalid flavour");
        return 1;
    }

    nec_print(NEC_INFO, "Setting flavour to %s", flavours[i_flavour - 1]);
    nec_write_file("res\\md_flavour", flavours[i_flavour - 1]);

    return 0;
}

int parse_md(char *input, char *output_type, char *specific) {
    if (check_requirements() == 1) {
        nec_print(NEC_ERROR, "Permission denied!\n");
        return 1;
    }

    if (validate_input(input, output_type, specific) == 1) {
        nec_print(NEC_ERROR, "Invalid input");
        return 1;
    }

    nec_print(NEC_INFO, "Parsing markdown file...");
    char *md = nec_read_file(input);
    if (md == NULL) {
        nec_print(NEC_ERROR, "Could not read input not read file");
        free(md);
        return 1;
    }

    if (md_to_html(input, md) == 1) {
        nec_print(NEC_ERROR, "Could not convert markdown to html");
        free(md);
        return 1;
    }

    if (generate_css() == 1) {
        nec_print(NEC_ERROR, "Could not generate css");
        free(md);
        return 1;
    }

    if (inject_css(input) == 1) {
        nec_print(NEC_ERROR, "Could not inject css");
        free(md);
        return 1;
    }

    return 0;
}

int check_requirements() {
    nec_print(NEC_INFO, "Checking requirements...");

    char *cmd = "npm ls -g";
    FILE *fp;

    int result = handle_requirement_check(fp, cmd);

    if (result != 0) {
        nec_print(NEC_INFO, "Package 'github-markdown-css' is not installed globally. Installing...");

        #if defined(_WIN32) || defined(_WIN64)
            system("npm install -g github-markdown-css >> NULL 2>&1");
        #else
            system("sudo npm install -g github-markdown-css >> /dev/null 2>&1");
        #endif

        result = handle_requirement_check(fp, cmd);

        if (result != 0) {
            return 1;
        } else {
            nec_print(NEC_INFO, "'github-markdown-css' installed successfully.");
        }
    } else {
        nec_print(NEC_INFO, "'github-markdown-css' is already installed.");
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
        nec_print(NEC_ERROR, "Failed running commands to check requirements.");
        pclose(fp);
        exit(1);
    }

    return strcmp(fgets(buffer, sizeof(buffer), fp), "") == 0;
}

int validate_input(char *input, char *output, char *specific) {
    if (strcmp(input, "") == 0) {
        nec_print(NEC_ERROR, "No input file specified.");
        return 1;
    }
    if (strcmp(output, "") == 0) {
        nec_print(NEC_ERROR, "No output type specified.");
        return 1;
    }
    if (strcmp(specific, "") == 0) {
        nec_print(NEC_ERROR, "No specific output type specified.");
        return 1;
    }
    return 0;
}

int md_to_html(char *filename, char *input) {
    nec_print(NEC_INFO, "Creating html file...");

    int commandLength = 16384;
    char *command = malloc(commandLength);

    strtok(filename, ".");

    if (command == NULL) {
        nec_print(NEC_ERROR, "Could not allocate memory.");
        return 1;
    }

    char *parsed_filename = malloc(strlen(filename) + 5);
    if (parsed_filename == NULL) {
        nec_print(NEC_ERROR, "Could not allocate memory.");
        return 1;
    }

    snprintf(parsed_filename, strlen(filename) + 32, "%s_tmp.html", filename);


#if defined(_WIN32) || defined(_WIN64)
    snprintf(command, commandLength,
             "gh api `"
             "  --method POST `"
             "  -H \"Accept: application/vnd.github+json\" `"
             "  -H \"X-GitHub-Api-Version: 2022-11-28\" `"
             "  /markdown `"
             "  -f text=\"%s\" `"
             "  -f mode=\"gfm\" | Out-File -FilePath \"%s\"",
             input, parsed_filename);
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
    nec_print(NEC_INFO, "Injecting css...");

    char *html_file_name = malloc(strlen(filename) + 32);
    if (html_file_name == NULL) {
        nec_print(NEC_ERROR, "Could not allocate memory.");
        return 1;
    }

    snprintf(html_file_name, strlen(filename) + 32, "%s_tmp.html", filename);

    char *html_file = nec_read_file(html_file_name);
    if (html_file == NULL) {
        nec_print(NEC_ERROR, "Could not read html file");
        return 1;
    }

    printf("%s\n", html_file);

    char *html_structure = malloc(strlen(html_file) + 1024);
    if (html_structure == NULL) {
        nec_print(NEC_ERROR, "Could not allocate memory.");
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
        snprintf(cmd, strlen(html_file_name) + 32, "del %s_tmp.html", filename);
        //system(cmd);
    #else
        snprintf(cmd, strlen(html_file_name) + 32, "rm %s_tmp.html", filename);
        system(cmd);
    #endif

    nec_write_file(html_file_name, html_structure);

    return 0;
}

int generate_css() {
    nec_print(NEC_INFO, "Generating css file...");

    return 0;
}