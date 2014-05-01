#include <CoreServices/CoreServices.h>

#include "lib.h"

void callbackFunction(void *target,
                      const char *path,
                      FSEventStreamEventFlags eventFlags,
                      FSEventStreamEventId eventId)
{
    printf("Change %s, %llu in %s, flags %lu\n",
           (const char *)target,
           eventId,
           path,
           (unsigned long)eventFlags);
}

int main(int argc, char **argv) {
    FSEventStreamRef stream;

    char cwd[1024];
    getcwd(cwd, sizeof(cwd));

    stream = createStream(cwd, 3.0, "test", &callbackFunction);
    scheduleStreamInRunLoop(stream);
}
