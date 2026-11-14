include(CTest)

add_executable(CsScriptSmoke "")

set_target_properties(CsScriptSmoke
   PROPERTIES
   RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/test"
)

target_link_libraries(CsScriptSmoke
   PUBLIC
   CsCore
   CsScript
)

target_sources(CsScriptSmoke
   PRIVATE
   ${CMAKE_CURRENT_SOURCE_DIR}/smoke_script.cpp
)

add_test(
   NAME CsScriptSmoke
   COMMAND CsScriptSmoke
)
