#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "log_interface.h"

static void (*rs_cb)(enum retro_log_level level, const char *fmt);

static void core_log_cb(enum retro_log_level level, const char *fmt, ...)
{
    va_list args;
    char buffer[4096];

    va_start(args, fmt);
    vsnprintf(buffer, sizeof(buffer), fmt, args);
    va_end(args);

    // envia o resultado para o rust
    rs_cb(level, buffer);
}

void configure_log_interface(RsCbT rs_cb_log, void *data)
{
    rs_cb = rs_cb_log;

    struct retro_log_callback *cb = (struct retro_log_callback *)data;
    cb->log = core_log_cb;
}

void set_variable_value_as_null(void *data)
{
    struct retro_variable *var = (struct retro_variable *)data;

    var->value = NULL;
}

bool set_new_value_variable(void *data, const char *new_value)
{
    struct retro_variable *var = (struct retro_variable *)data;

    var->value = malloc(strlen(new_value) + 1);

    if (var->value != NULL)
    {
        strcpy(var->value, new_value);
        return true;
    }
    else
    {
        var->value = NULL;
        return false;
    }
}

void set_directory(void *data, const char *new_directory)
{
    const char **dir = (const char **)data;

    *dir = malloc(strlen(new_directory) + 1);

    if (*dir != NULL)
    {
        strcpy(*dir, new_directory);
    }
    else
    {
        *dir = NULL;
    }
}