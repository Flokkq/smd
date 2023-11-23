#ifndef SMD_NEC_H
#define SMD_NEC_H

#define BLUE_CODE	"\033[34m"
#define BOLD_CODE	"\033[1m"
#define BOLD_GREEN_CODE	"\033[1;32m"
#define GREEN_CODE	"\033[32m"
#define MAGENTA_CODE	"\033[35m"
#define NORMAL_CODE	"\033[0m"
#define RED_CODE	"\033[31m"
#define YELLOW_CODE	"\033[33m"

#define E "ERROR"
#define W "WARNING"
#define I "INFO"

void noc_print(char* type, char* message, ...);
#endif
