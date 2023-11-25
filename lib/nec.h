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

#define NEC_NORMAL "NORMAL"
#define NEC_ERROR "ERROR"
#define NEC_WARNING "WARNING"
#define NEC_INFO "INFO"

typedef enum {
    NEC_JSON_NULL,
    NEC_JSON_INT,
    NEC_JSON_FLOAT,
    NEC_JSON_STRING,
    NEC_JSON_ARRAY,
    NEC_JSON_OBJECT
} nec_json_type;

typedef struct s_nec_json s_nec_json;
struct s_nec_json {
    nec_json_type type;
    union {
        int int_value;
        float float_value;
        char *string_value;
        struct {
            s_nec_json *items;
            size_t size;
        } array;
        struct {
            char **keys;
            s_nec_json *values;
            size_t size;
        } object;
    } data;
};

void nec_print(char* type, const char* message, ...);
char *nec_read_file(const char *filename);
void nec_write_file(const char *filename, char *content);
char *nec_stringify_json(s_nec_json *json);
s_nec_json parse_json(char *json_string);


#endif
