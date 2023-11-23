#include <stdio.h>
#include <string.h>
#include <stdarg.h>
#include "nec.h"

#define BLUE_CODE	"\033[34m"
#define BOLD_CODE	"\033[1m"
#define BOLD_GREEN_CODE	"\033[1;32m"
#define GREEN_CODE	"\033[32m"
#define MAGENTA_CODE	"\033[35m"
#define NORMAL_CODE	"\033[0m"
#define RED_CODE	"\033[31m"
#define YELLOW_CODE	"\033[33m"

 void noc_print(char* type, char* message, ...) {
    char *color = "", *reset = "\033[0m";
    if (strcmp(type, "ERROR") == 0) {
        color = RED_CODE;
    } else if (strcmp(type, "WARNING") == 0) {
        color = YELLOW_CODE;
    } else if (strcmp(type, "INFO") == 0) {
        color = GREEN_CODE;
    } else {
        color = NORMAL_CODE;
    }
    printf("%s%s%s %s\n",color, type, reset, message);
}