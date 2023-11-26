#include <stdio.h>
#include <string.h>
#include <stdarg.h>
#include <stdlib.h>
#include "nec.h"

#define BLUE_CODE    "\033[34m"
#define BOLD_CODE    "\033[1m"
#define BOLD_GREEN_CODE    "\033[1;32m"
#define GREEN_CODE    "\033[32m"
#define MAGENTA_CODE    "\033[35m"
#define NORMAL_CODE    "\033[0m"
#define RED_CODE    "\033[31m"
#define YELLOW_CODE    "\033[33m"

void nec_print(char *type, const char *format, ...) {
    char *color = "", *reset = "\033[0m";
    va_list args;

    if (strcmp(type, "ERROR") == 0) {
        color = RED_CODE;
    } else if (strcmp(type, "WARNING") == 0) {
        color = YELLOW_CODE;
    } else if (strcmp(type, "INFO") == 0) {
        color = GREEN_CODE;
    } else if (strcmp(type, "NORMAL") == 0) {
        color = NORMAL_CODE;
        type = "";
    } else {
        color = NORMAL_CODE;
    }

    printf("%s%s%s ", color, type, reset);

    va_start(args, format);
    vprintf(format, args);
    va_end(args);

    printf("\n");
}

char* nec_stdin() {
    size_t size_increment = 10; // Increment size for buffer
    char *buffer = malloc(size_increment);
    char *current_position = buffer;
    size_t current_size = size_increment;
    int character;

    if (!buffer) {
        nec_print(NEC_ERROR, "Error allocating memory for input.");
        return NULL;
    }

    while (1) {
        character = getchar();

        if (character == '\n' || character == EOF) {
            *current_position = '\0';
            break;
        } else {
            *current_position = character;
            current_position++;

            if ((size_t)(current_position - buffer) == current_size) {
                current_size += size_increment;
                char *new_buffer = realloc(buffer, current_size);
                if (!new_buffer) {
                    nec_print(NEC_ERROR, "Error allocating memory for input.");
                    free(buffer);
                    return NULL;
                }
                current_position = new_buffer + (current_position - buffer);
                buffer = new_buffer;
            }
        }
    }

    char *shrunk_buffer = realloc(buffer, strlen(buffer) + 1);
    if (shrunk_buffer) {
        buffer = shrunk_buffer;
    }

    return buffer;
}

char *nec_read_file(const char *filename) {
    FILE *file;
    char *buffer;
    long file_size;

    file = fopen(filename, "rb");
    if (file == NULL) {
        nec_print(NEC_ERROR, "Error opening file %s", filename);
        return NULL;
    }

    fseek(file, 0, SEEK_END);
    file_size = ftell(file);
    rewind(file);

    buffer = (char *)malloc(sizeof(char) * (file_size + 1));
    if (buffer == NULL) {
        nec_print(NEC_ERROR, "Error allocating memory for file %s", filename);
        fclose(file);
        return NULL;
    }

    if (fread(buffer, sizeof(char), file_size, file) != file_size) {
        nec_print(NEC_ERROR, "Error reading file %s", filename);
        free(buffer);
        fclose(file);
        return NULL;
    }

    buffer[file_size] = '\0';

    fclose(file);
    return buffer;
}

void nec_write_file(const char *filename, char *content) {
    FILE *fp;
    fp = fopen(filename, "w");

    if (fp == NULL) {
        nec_print(NEC_ERROR, "Could not open file %s", filename);
        exit(1);
    }

    fprintf(fp, "%s", content);
}

char *nec_stringify_json(s_nec_json *json) {
    // TODO: kys while implementing this
    return NULL;
}

s_nec_json parse_json(char *json_string) {
    // TODO: kys while implementing this
    return (s_nec_json) {NEC_JSON_NULL};
}