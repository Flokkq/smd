#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#include "lib/smd.h"
#include "lib/nec.h"

#if !defined(_WIN32) && !defined(_WIN64)

#include <unistd.h>

#endif

#define VERSION "0.0.1"

char *absolute_path(char *filename);


int main(int argc, char **argv) {
    if (argc < 2 || argc > 7) {
        printf("Invalid arguments! \nPlease use smd --help for more information\n");
        return 1;
    }

    switch (argc) {
        case 2:
            if (strcmp(argv[1], "--help") == 0) {
                printf("%sUsage:%s %ssmd%s %s[input_file] [output_type] [*specific_type]%s \n", RED_CODE, NORMAL_CODE,
                       BLUE_CODE, NORMAL_CODE, MAGENTA_CODE, NORMAL_CODE);
                printf("%sOptions:%s\n", RED_CODE, NORMAL_CODE);
                printf("  %s--help%s\t\t\tDisplay this information\n", MAGENTA_CODE, NORMAL_CODE);
                printf("  %s--init%s\t\t\tInitialize smd. Needed for the first use.\n", MAGENTA_CODE, NORMAL_CODE);
                printf("  %s--flavour%s\t\t\tSet flavour for the md files.\n", MAGENTA_CODE, NORMAL_CODE);
                printf("  %s--version%s\t\t\tDisplay version information\n", MAGENTA_CODE, NORMAL_CODE);
                printf("  %s--output [file]%s\t\tSpecify output type\n", MAGENTA_CODE, NORMAL_CODE);
                printf("  %s--input [file]%s\t\tSpecify input file\n", MAGENTA_CODE, NORMAL_CODE);
                printf("  %s--specific [type]%s\t\tSpecify specific output type\n", MAGENTA_CODE, NORMAL_CODE);
                return 0;
            } else if (strcmp(argv[1], "--version") == 0) {
                printf("%sSMD%s version %s\n", BLUE_CODE, NORMAL_CODE, VERSION);
                return 0;
            } else if (strcmp(argv[1], "--init") == 0) {
                printf("Initializing %ssmd%s...", BLUE_CODE, NORMAL_CODE);  
                return 0;
            } else if (strcmp(argv[1], "--flavour") == 0) {
                printf("What flavour do you want to use?\n");
                printf("  %s1%s > dark\n", MAGENTA_CODE, NORMAL_CODE);
                printf("  %s2%s > light\n", MAGENTA_CODE, NORMAL_CODE);
                printf("  %s3%s > auto\n", MAGENTA_CODE, NORMAL_CODE);
                printf("flavour: ");

                char *flavour = nec_stdin();
                if (flavour == NULL) {
                    nec_print(NEC_ERROR, "Could not read flavour");
                    return 1;
                }

                set_md_flavour(flavour);
                return 0;
            } else {
                printf("Invalid arguments! \nPlease use %ssmd%s %s--help%s for more information\n", BLUE_CODE,
                       NORMAL_CODE, MAGENTA_CODE, NORMAL_CODE);
                return 1;
            }
        case 5:
            if (strcmp(argv[1], "--input") == 0) {
                if (strcmp(argv[3], "--output") == 0) {
                    return parse_md(argv[2], argv[4], "x");
                } else {
                    printf("Invalid arguments! \nPlease use %ssmd%s %s--help%s for more information\n", BLUE_CODE,
                           NORMAL_CODE, MAGENTA_CODE, NORMAL_CODE);
                    return 1;
                }
            } else if (strcmp(argv[1], "--output") == 0) {
                if (strcmp(argv[3], "--input") == 0) {
                    return parse_md(absolute_path(argv[2]), argv[4], "x");
                } else {
                    printf("Invalid arguments! \nPlease use %ssmd%s %s--help%s for more information\n", BLUE_CODE,
                           NORMAL_CODE, MAGENTA_CODE, NORMAL_CODE);
                    return 1;
                }
            } else {
                printf("Invalid arguments! \nPlease use %ssmd%s %s--help%s for more information\n", BLUE_CODE,
                       NORMAL_CODE, MAGENTA_CODE, NORMAL_CODE);
                return 1;
            }
        case 7:
            if (strcmp(argv[1], "--input") == 0) {
                if (strcmp(argv[3], "--output") == 0) {
                    if (strcmp(argv[5], "--specific") == 0) {
                        if (strcmp(argv[6], "img") == 0) {
                            return parse_md(argv[2], argv[4], argv[6]);
                        } else {
                            printf("Invalid arguments! \nOnly if the output type is img the --specific option is available",
                                   BLUE_CODE, NORMAL_CODE, MAGENTA_CODE, NORMAL_CODE);
                            return 1;

                        }
                    } else {
                        printf("Invalid arguments! \nPlease use %ssmd%s %s--help%s for more information\n", BLUE_CODE,
                               NORMAL_CODE, MAGENTA_CODE, NORMAL_CODE);
                        return 1;
                    }
                } else {
                    printf("Invalid arguments! \nPlease use %ssmd%s %s--help%s for more information\n", BLUE_CODE,
                           NORMAL_CODE, MAGENTA_CODE, NORMAL_CODE);
                    return 1;
                }
            } else if (strcmp(argv[1], "--output") == 0) {
                if (strcmp(argv[3], "--input") == 0) {
                    if (strcmp(argv[5], "--specific") == 0) {
                        return parse_md(absolute_path(argv[2]), argv[4], argv[6]);
                    } else {
                        printf("Invalid arguments! \nPlease use %ssmd%s %s--help%s for more information\n", BLUE_CODE,
                               NORMAL_CODE, MAGENTA_CODE, NORMAL_CODE);
                        return 1;
                    }
                } else {
                    printf("Invalid arguments! \nPlease use %ssmd%s %s--help%s for more information\n", BLUE_CODE,
                           NORMAL_CODE, MAGENTA_CODE, NORMAL_CODE);
                    return 1;
                }
            } else {
                printf("Invalid arguments! \nPlease use %ssmd%s %s--help%s for more information\n", BLUE_CODE,
                       NORMAL_CODE, MAGENTA_CODE, NORMAL_CODE);
                return 1;
            }
        default:
            printf("%sInvalid arguments!%s \nPlease use %ssmd%s %s--help%s for more information\n", RED_CODE,
                   NORMAL_CODE BLUE_CODE, NORMAL_CODE, MAGENTA_CODE, NORMAL_CODE);
            return 1;
    }
}

char *absolute_path(char *filename) {
    char *buffer;
    char *abs_path;

#if defined(_WIN32) || defined(_WIN64)
    abs_path = _fullpath(NULL, filename, _MAX_PATH);
#else
    buffer = getcwd(NULL, 0);
    abs_path = (char *) malloc(strlen(buffer) + strlen(filename) + 2); // +2 for the slash and null terminator
    sprintf(abs_path, "%s/%s", buffer, filename);
    free(buffer);
#endif

    printf("path: %s\n", abs_path);
    if (abs_path == NULL) {
        nec_print(NEC_ERROR, "Could not get absolute path");
        return NULL;
    }
    return abs_path;
}