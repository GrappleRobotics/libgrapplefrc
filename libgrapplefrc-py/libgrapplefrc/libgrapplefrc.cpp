#ifdef _WIN32
  #pragma comment(lib, "Ws2_32.lib")
  #pragma comment(lib, "userenv.lib")
  #pragma comment(lib, "psapi.lib")
  #pragma comment(lib, "bcrypt.lib")
  #pragma comment(lib, "ntdll.lib")
  #pragma comment(lib, "advapi32.lib")
#endif

#include <rpygen_wrapper.hpp>

RPYBUILD_PYBIND11_MODULE(m) { initWrapper(m); }