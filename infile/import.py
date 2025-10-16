# test_completo.py
print("=== TEST DE ESTRUCTURA ===")

# 1. Test imports desde submódulos
try:
    from suma_ulsa.conversions import NumberConverter, binary_to_decimal
    from suma_ulsa.boolean_algebra import BooleanExpr
    from suma_ulsa.networking import SubnetCalculator
    
    print("✓ Importaciones desde submódulos: OK")
    
    # Test funcionalidad conversiones
    converter = NumberConverter(42)
    binary_result = converter.to_binary()
    print(f"✓ NumberConverter.to_binary(): {binary_result}")
    
    # Test función directa
    decimal_result = binary_to_decimal("1010")
    print(f"✓ binary_to_decimal('1010'): {decimal_result}")
    
    # Test álgebra booleana
    expr = BooleanExpr("A AND B")
    print(f"✓ BooleanExpr creado: {expr}")
    result = expr.evaluate({'A': True, 'B': False})
    print(f"✓ BooleanExpr.evaluate(): {result}")
    
    print("🎉 TODAS LAS PRUEBAS PASARON!")

    calc = SubnetCalculator("192.168.1.1", 5)
    print(f"✓ SubnetCalculator creado: {calc.summary()}")

except ImportError as e:
    print(f"✗ Error de importación: {e}")
    
    # Debug detallado
    print("\n=== DEBUG DETALLADO ===")
    import suma_ulsa.suma_ulsa as rust
    print("Módulos en Rust:", [x for x in dir(rust) if not x.startswith('_')])
    
    try:
        import suma_ulsa.conversions as conv
        print("Contenido de conversions:", [x for x in dir(conv) if not x.startswith('_')])
    except Exception as e:
        print(f"Error accediendo conversions: {e}")
        
except Exception as e:
    print(f"✗ Error de ejecución: {e}")
    import traceback
    traceback.print_exc()