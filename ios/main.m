#import <UIKit/UIKit.h>

// Forward declaration of SDL_main since touchHLE uses SDL2
extern int SDL_main(int argc, char *argv[]);

int main(int argc, char *argv[]) {
    // SDL2 on iOS hooks into the application lifecycle automatically.
    // We just need to invoke SDL_main.
    @autoreleasepool {
        // Set environment variables or paths if needed for touchHLE to find the .ipa
        NSString *documentsDirectory = [NSSearchPathForDirectoriesInDomains(NSDocumentDirectory, NSUserDomainMask, YES) firstObject];
        setenv("TOUCHHLE_APP_DIR", [documentsDirectory UTF8String], 1);
        
        return SDL_main(argc, argv);
    }
}
