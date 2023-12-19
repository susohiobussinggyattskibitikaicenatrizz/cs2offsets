/*
 * Created using https://github.com/a2x/cs2-dumper
 * Tue, 19 Dec 2023 01:01:50 +0000
 */

#pragma once

#include <cstddef>

namespace CAnimScriptBase {
    constexpr std::ptrdiff_t m_bIsValid = 0x8; // bool
}

namespace EmptyTestScript { // CAnimScriptBase
    constexpr std::ptrdiff_t m_hTest = 0x10; // CAnimScriptParam<float>
}