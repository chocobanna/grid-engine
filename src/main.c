#include <SDL3/SDL.h>
#include <stdio.h>

int main(void) {
    if (!SDL_Init(SDL_INIT_VIDEO)) {
        const char *err = SDL_GetError();
        fprintf(stderr, "SDL_Init failed: %s\n", (err && *err) ? err : "(no error string)");
        return 1;
    }

    SDL_Window *win = SDL_CreateWindow("SDL3 window", 800, 600, 0);
    if (!win) {
        fprintf(stderr, "SDL_CreateWindow failed: %s\n", SDL_GetError());
        SDL_Quit();
        return 1;
    }

    SDL_Renderer *ren = SDL_CreateRenderer(win, NULL);
    if (!ren) {
        fprintf(stderr, "SDL_CreateRenderer failed: %s\n", SDL_GetError());
        SDL_DestroyWindow(win);
        SDL_Quit();
        return 1;
    }

    // IMPORTANT ON WAYLAND: present a buffer so the window becomes visible.
    SDL_SetRenderDrawColor(ren, 20, 20, 20, 255);
    SDL_RenderClear(ren);
    SDL_RenderPresent(ren);

    int running = 1;
    while (running) {
        SDL_Event e;
        while (SDL_PollEvent(&e)) {
            if (e.type == SDL_EVENT_QUIT) running = 0;
        }

        // keep presenting so compositors don't treat you as dead weight
        SDL_RenderPresent(ren);
        SDL_Delay(16);
    }

    SDL_DestroyRenderer(ren);
    SDL_DestroyWindow(win);
    SDL_Quit();
    return 0;
}
