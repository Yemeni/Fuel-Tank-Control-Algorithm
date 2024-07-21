Feature: Fuel tank control algorithm

  Scenario: Preventing overflow
    Given a fuel tank with a capacity of 100 liters
    When the current fuel level is 90 liters
    And 15 liters of fuel is added
    Then the fuel tank should indicate an overflow warning

  Scenario: Normal operation
    Given a fuel tank with a capacity of 100 liters
    When the current fuel level is 50 liters
    And 30 liters of fuel is added
    Then the fuel tank should not indicate an overflow warning