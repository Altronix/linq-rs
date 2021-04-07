#include "log.h"

#define LOG_RESET "\x1b[0m"
#define LOG_BOLD "\x1b[1m"
#define LOG_BOLD_OFF "\x1b[21m"
#define LOG_BLINK "\x1b[5m"
#define LOG_BLINK_OFF "\x1b[25m"
#define LOG_UNDERLINE "\x1b[4m"
#define LOG_UNDERLINE_OFF "\x1b[24m"

#define LOG_BLACK "\x1b[30m"
#define LOG_RED "\x1b[31m"
#define LOG_GREEN "\x1b[32m"
#define LOG_YELLOW "\x1b[33m"
#define LOG_BLUE "\x1b[34m"
#define LOG_MAGENTA "\x1b[35m"
#define LOG_CYAN "\x1b[36m"
#define LOG_WHITE "\x1b[37m"
#define LOG_DEFAULT "\x1b[39m"
#define LOG_GRAY "\x1b[90m"
#define LOG_LIGHT_RED "\x1b[91m"
#define LOG_LIGHT_GREEN "\x1b[92m"
#define LOG_LIGHT_YELLOW "\x1b[93m"
#define LOG_LIGHT_BLUE "\x1b[94m"
#define LOG_LIGHT_MAGENTA "\x1b[95m"
#define LOG_LIGHT_CYAN "\x1b[96m"
#define LOG_LIGHT_WHITE "\x1b[97m"

#define LOG_BACKGROUND_BLACK "\x1b[40m"
#define LOG_BACKGROUND_RED "\x1b[41m"
#define LOG_BACKGROUND_GREEN "\x1b[42m"
#define LOG_BACKGROUND_YELLOW "\x1b[43m"
#define LOG_BACKGROUND_BLUE "\x1b[44m"
#define LOG_BACKGROUND_MAGENTA "\x1b[45m"
#define LOG_BACKGROUND_CYAN "\x1b[46m"
#define LOG_BACKGROUND_WHITE "\x1b[47m"
#define LOG_BACKGROUND_DEFAULT "\x1b[49m"
#define LOG_BACKGROUND_LIGHT_GRAY "\x1b[100m"
#define LOG_BACKGROUND_LIGHT_RED "\x1b[101m"
#define LOG_BACKGROUND_LIGHT_GREEN "\x1b[102m"
#define LOG_BACKGROUND_LIGHT_YELLOW "\x1b[103m"
#define LOG_BACKGROUND_LIGHT_BLUE "\x1b[104m"
#define LOG_BACKGROUND_LIGHT_MAGENTA "\x1b[105m"
#define LOG_BACKGROUND_LIGHT_CYAN "\x1b[106m"
#define LOG_BACKGROUND_LIGHT_WHITE "\x1b[107m"
#define ARROW LOG_WHITE "=> " LOG_RESET
#define ARROW_BW "=> "
#define TICK LOG_WHITE "%5d " LOG_RESET
#define TICK_BW "%5d "
#define LEVEL "%s%s " LOG_RESET
#define LEVEL_BW "%.0s%s "
#define LINE LOG_MAGENTA "%14s:%04d " LOG_RESET
#define LINE_BW "%14s:%04d "
#define CAT LOG_LIGHT_CYAN "[%5s] " LOG_RESET
#define CAT_BW "[%.5s] "
#define FMT TICK LEVEL LINE CAT ARROW
#define FMT_BW TICK_BW LEVEL_BW LINE_BW CAT ARROW_BW

static FILE* logger = NULL;
static bool color = true;
static const char* level_names[] = { "TRACE", "DEBUG", "INFO ",
                                     "WARN ", "ERROR", "FATAL" };
static const char* level_colors[] = { LOG_LIGHT_BLUE, LOG_CYAN, LOG_GREEN,
                                      LOG_YELLOW,     LOG_RED,  LOG_MAGENTA };

static log_callback_fn log_callback = NULL;
static void* log_callback_context = NULL;

void
log_set_callback_fn(log_callback_fn fn, void* ctx)
{
    log_callback_context = ctx;
    log_callback = fn;
}

void
log_set_fd(FILE* f)
{
    logger = f;
}

void
log_set_color(bool c)
{
    color = c;
}

void
log_log(
    E_LOG_LEVEL level,
    const char* file,
    int line,
    const char* cat,
    const char* fmt,
    ...)
{
    log_callback_s callback = { 0 };
    size_t flen = strlen(file) - 1;
    while ((!(file[flen] == '/' || file[flen] == '\\')) && flen) { flen--; }
    file = flen > 1 ? &file[flen + 1] : file;
    va_list args;
    FILE* out = logger ? logger : stdout;
    uint32_t tick = 0;

    if (log_callback) {
        callback.file = file;
        callback.line = line;
        callback.tick = tick;
        callback.level = level_names[level];
        callback.category = cat;
        callback.context = log_callback_context;
        va_start(args, fmt);
        vsnprintf(callback.message, sizeof(callback.message), fmt, args);
        va_end(args);
        log_callback(&callback);
        if (level == LINQ_FATAL) assert_fn(callback.message);
    } else {
        fprintf(
            out,
            color ? FMT : FMT_BW,
            tick,
            level_colors[level],
            level_names[level],
            file,
            line,
            cat);

        va_start(args, fmt);
        vfprintf(out, fmt, args);
        va_end(args);
        fprintf(out, "\n");
        fflush(out);
        if (level == LINQ_FATAL) { assert_fn(!"See [FATAL] above"); }
    }
}

