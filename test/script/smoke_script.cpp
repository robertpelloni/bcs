// Smoke test for the revived CsScript engine (Stage A).
//
// Guards the three defects that kept the engine from running at all:
//   1. PageBlock.cpp missing from the link (QTWTF::pageSize unresolved).
//   2. Masquerading ClassInfo entries with no parent chain, tripping the
//      JSObject constructor inherits() assertion at engine startup.
//   3. JSGlobalData teardown order: Strong<Structure> handles must be
//      cleared (clearBuiltinStructures) before heap.destroy(), or the
//      process crashes on exit after otherwise-successful evaluation.
//
// Built only when WITH_SCRIPT=ON (see top-level CMakeLists). Run via
// ctest (CsScriptSmoke) or directly; prints SCRIPT SMOKE OK on success.

#include <QtCore/QCoreApplication>
#include <QtScript/QScriptEngine>
#include <QtScript/QScriptValue>

#include <iostream>

int main(int argc, char **argv)
{
   QCoreApplication app(argc, argv);
   QScriptEngine engine;

   QScriptValue result = engine.evaluate("1 + 2");
   std::cout << "1 + 2 = " << result.toNumber() << std::endl;
   if (result.toNumber() != 3) {
      std::cout << "FAILED: arithmetic" << std::endl;
      return 1;
   }

   QScriptValue fn = engine.evaluate("(function(x) { return x * x; })(7)");
   std::cout << "7 * 7 = " << fn.toNumber() << std::endl;
   if (fn.toNumber() != 49) {
      std::cout << "FAILED: function call" << std::endl;
      return 1;
   }

   QScriptValue str = engine.evaluate("'BTK' + ' Script'");
   std::cout << "concat = " << str.toString().toStdString() << std::endl;
   if (str.toString() != "BTK Script") {
      std::cout << "FAILED: string concat" << std::endl;
      return 1;
   }

   if (engine.hasUncaughtException()) {
      std::cout << "FAILED: uncaught exception" << std::endl;
      return 1;
   }

   std::cout << "SCRIPT SMOKE OK" << std::endl;
   return 0;
}
