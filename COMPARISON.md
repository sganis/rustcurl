# Feature Comparison: rustcurl vs orion/kerbhttp

## Overview

Both projects use the `curl` crate for HTTP requests with enterprise authentication, but have different feature sets and use cases.

## Feature Matrix

| Feature | rustcurl | kerbhttp | Notes |
|---------|----------|----------|-------|
| **HTTP Methods** |
| GET, POST, PUT, DELETE, PATCH | ✅ | ✅ | Both support |
| HEAD, OPTIONS | ✅ | ❌ | rustcurl only |
| Custom methods | ✅ | ❌ | rustcurl only |
| **Authentication** |
| Kerberos/SPNEGO | ✅ | ✅ | Primary feature for both |
| NTLM | ✅ | ❌ | rustcurl only |
| Basic Auth (user:pass) | ✅ | ❌ | rustcurl only |
| Bearer Token | ✅ | ❌ | rustcurl only |
| **Request Features** |
| Headers | ✅ | ✅ | Both support |
| Query parameters | ✅ | ✅ | Both support |
| Body (text/json/bytes) | ✅ | ✅ | Both support |
| Custom User-Agent | ✅ | ❌ | rustcurl has default Edge UA |
| **TLS/SSL** |
| Insecure mode (-k) | ✅ | ✅ | Both support |
| CA certificate bundle | ✅ | ✅ | Both support |
| Accept invalid hostnames | ❌ | ✅ | kerbhttp only |
| **Proxy** |
| HTTP/HTTPS proxy | ✅ | ✅ | Both support |
| Proxy authentication | ✅ | ✅ | Both support |
| No-proxy list | ✅ | ✅ | Both support |
| **Response Handling** |
| Headers capture | ✅ | ✅ | Both support |
| Body capture | ✅ | ✅ | Both support |
| JSON auto-detection | ❌ | ✅ | kerbhttp auto-parses JSON |
| Output to file | ✅ | ❌ | rustcurl only |
| **Timeouts** |
| Connect timeout | ✅ | ❌ | rustcurl only |
| Total timeout | ✅ | ✅ | Both support |
| **Redirects** |
| Follow redirects | ✅ | ❌ | rustcurl only |
| Max redirects limit | ✅ | ❌ | rustcurl only |
| **Cookies** |
| Read cookies from file | ✅ | ❌ | rustcurl only |
| Write cookies to jar | ✅ | ❌ | rustcurl only |
| **Compression** |
| Accept-Encoding | ✅ | ❌ | rustcurl only |
| **Advanced Features** |
| Custom DNS resolution (--resolve) | ✅ | ❌ | rustcurl only |
| Verbose output | ✅ | ❌ | rustcurl only |
| Silent mode | ✅ | ❌ | rustcurl only |
| Timing breakdown | ✅ | ⚠️ | rustcurl: full breakdown; kerbhttp: total only |
| Policy enforcement | ❌ | ✅ | kerbhttp only |
| Error classification | ❌ | ✅ | kerbhttp categorizes errors |
| **Architecture** |
| CLI tool | ✅ | ❌ | rustcurl only |
| Library/API | ❌ | ✅ | kerbhttp only |
| Synchronous | ✅ | ✅ | Both blocking (kerbhttp wrapped in async) |
| Async wrapper | ❌ | ✅ | kerbhttp uses tokio::spawn_blocking |
| Tauri IPC integration | ❌ | ✅ | kerbhttp designed for Tauri |

## Key Differences

### rustcurl Strengths:
1. **Complete curl feature parity** - supports almost all curl flags
2. **Multiple auth methods** - NTLM, basic, bearer token
3. **Cookie handling** - read/write cookie jars
4. **Redirect control** - follow limits, max redirects
5. **Compression** - automatic content encoding
6. **CLI tool** - ready-to-use command-line interface
7. **Output modes** - silent, head-only, verbose

### kerbhttp Strengths:
1. **Policy enforcement** - security controls for negotiate/insecure
2. **JSON-first** - auto-detects and parses JSON responses
3. **Tauri integration** - designed for IPC with frontend
4. **Error classification** - categorized error types
5. **Async wrapper** - non-blocking via spawn_blocking
6. **Audit logging** - automatic request logging

## Missing Features in kerbhttp

### High Priority (Should Add):
1. ❌ **User-Agent header** - Many APIs require this (like Wikipedia)
2. ❌ **Follow redirects** - Common requirement for APIs
3. ❌ **Bearer token auth** - Standard for modern REST APIs
4. ❌ **Basic auth** - Still widely used

### Medium Priority (Nice to Have):
5. ❌ **Cookie support** - Useful for session-based APIs
6. ❌ **Compression** - Reduces bandwidth
7. ❌ **HEAD/OPTIONS methods** - Used for API discovery
8. ❌ **Connect timeout** - Separate from total timeout

### Low Priority (Edge Cases):
9. ❌ **NTLM auth** - Less common than Kerberos
10. ❌ **Custom DNS resolution** - Rare use case
11. ❌ **Verbose output** - Debugging feature
12. ❌ **Output to file** - Can be done in frontend

## Recommendations

### For kerbhttp:
1. **Add immediately:**
   - Default User-Agent (Edge browser string)
   - Follow redirects (with HTTP/1.1 default already set)
   - Bearer token authentication

2. **Consider adding:**
   - Basic authentication (user:pass)
   - Cookie support
   - HEAD/OPTIONS methods
   - Compression support

3. **Keep as-is:**
   - Policy enforcement (unique security feature)
   - JSON auto-detection (useful for Tauri)
   - Error classification (good UX)

### For rustcurl:
1. **Already complete** - Has all major curl features
2. **Consider adding:**
   - Error classification like kerbhttp
   - JSON auto-detection for response body

## Use Cases

### Use rustcurl when:
- Building CLI tools
- Need complete curl compatibility
- Multiple authentication methods required
- Cookie/session management needed

### Use kerbhttp when:
- Building Tauri apps
- Need Kerberos auth in a desktop app
- Policy enforcement required
- JSON APIs with TypeScript integration

## Conclusion

**rustcurl** is a complete curl wrapper with broad feature coverage.
**kerbhttp** is a focused library optimized for Kerberos auth in Tauri apps.

kerbhttp should adopt these critical features from rustcurl:
1. Default User-Agent
2. Bearer token auth
3. Follow redirects
4. Basic auth support
