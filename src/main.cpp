// main.cpp
#include <iostream>
#include <stdexcept>

#define GLFW_INCLUDE_NONE
#include <GLFW/glfw3.h>

#if defined(__linux__)
  #include <GL/gl.h>   // from mesa / libGL
#endif

static void glfw_error_cb(int code, const char* desc) {
  std::cerr << "GLFW error (" << code << "): " << desc << "\n";
}

int main() {
  glfwSetErrorCallback(glfw_error_cb);

  if (!glfwInit()) {
    std::cerr << "Failed to init GLFW.\n";
    return 1;
  }

  // Ask for an OpenGL context. GLFW will use Wayland backend if available.
  glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
  glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
  glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
  glfwWindowHint(GLFW_RESIZABLE, GLFW_TRUE);

  GLFWwindow* win = glfwCreateWindow(900, 600, "Wayland Window (GLFW)", nullptr, nullptr);
  if (!win) {
    glfwTerminate();
    std::cerr << "Failed to create window.\n";
    return 1;
  }

  glfwMakeContextCurrent(win);
  glfwSwapInterval(1); // vsync

  while (!glfwWindowShouldClose(win)) {
    int w, h;
    glfwGetFramebufferSize(win, &w, &h);
    glViewport(0, 0, w, h);

    // obnoxiously teal background so you know it's working
    glClearColor(0.05f, 0.75f, 0.70f, 1.0f);
    glClear(GL_COLOR_BUFFER_BIT);

    glfwSwapBuffers(win);
    glfwPollEvents();
  }

  glfwDestroyWindow(win);
  glfwTerminate();
  return 0;
}
