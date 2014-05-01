#include "lib.h"

struct CallbackWithTarget {
    void *target;
    StreamCallback callback;
};
typedef struct CallbackWithTarget CallbackWithTarget;

void callbackFunction(ConstFSEventStreamRef streamRef,
                      void *clientCallBackInfo,
                      size_t numEvents,
                      void *eventPaths,
                      const FSEventStreamEventFlags eventFlags[],
                      const FSEventStreamEventId eventIds[])
{
    size_t i;
    char **paths = eventPaths;
    CallbackWithTarget *stream;

    stream = (CallbackWithTarget *)clientCallBackInfo;
    for (i=0; i < numEvents; i++) {
        stream->callback(stream->target, paths[i], eventFlags[i], eventIds[i]);
    }
}

FSEventStreamRef createStream(const char *path,
                              double latency,
                              void *target,
                              StreamCallback callback)
{
    CFStringRef dirPath;
    CFArrayRef pathToWatch;
    FSEventStreamRef stream;
    FSEventStreamContext *context;
    CallbackWithTarget *cbWithTarget;

    dirPath = CFStringCreateWithCString(NULL, path, kCFStringEncodingUTF8);
    pathToWatch = CFArrayCreate(NULL, (const void **)&dirPath, 1, NULL);

    cbWithTarget = (CallbackWithTarget *)malloc(sizeof(CallbackWithTarget));
    cbWithTarget->target = target;
    cbWithTarget->callback = callback;

    context = (FSEventStreamContext *)malloc(sizeof(FSEventStreamContext));
    context->version = 0;
    context->info = cbWithTarget;
    context->retain = NULL;
    context->release = NULL;
    context->copyDescription = NULL;

    stream = FSEventStreamCreate(NULL,
                                 &callbackFunction,
                                 context,
                                 pathToWatch,
                                 kFSEventStreamEventIdSinceNow,
                                 latency,
                                 kFSEventStreamCreateFlagFileEvents);

    free(context);
    return stream;
}

bool scheduleStreamInRunLoop(FSEventStreamRef stream)
{
    FSEventStreamScheduleWithRunLoop(stream,
                                     CFRunLoopGetCurrent(),
                                     kCFRunLoopDefaultMode);
    if (!FSEventStreamStart(stream)) return false;
    CFRunLoopRun();

    return true;
}

void unscheduleStream(FSEventStreamRef stream)
{
    FSEventStreamStop(stream);
    FSEventStreamInvalidate(stream);
}

void destroyStream(FSEventStreamRef stream)
{
    FSEventStreamRelease(stream);
}
