LOCAL_PATH := $(call my-dir)

include $(CLEAR_VARS)
LOCAL_MODULE := drrust
LOCAL_SRC_FILES := /Users/kmurph/code/android-prj/app/jni/src/libdrrust.a
include $(PREBUILT_STATIC_LIBRARY)

include $(CLEAR_VARS)

LOCAL_MODULE := main

SDL_PATH := ../SDL2
SDL_IMAGE_PATH := ../SDL2_image

LOCAL_C_INCLUDES := $(LOCAL_PATH)/$(SDL_PATH)/include

# Add your application source files here...
LOCAL_SRC_FILES := happy.c

LOCAL_SHARED_LIBRARIES := SDL2 SDL2_image
LOCAL_STATIC_LIBRARIES := drrust

LOCAL_LDLIBS := -lGLESv1_CM -lGLESv2 -lOpenSLES -llog -landroid

include $(BUILD_SHARED_LIBRARY)