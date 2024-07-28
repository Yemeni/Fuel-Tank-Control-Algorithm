Feature: Control algorithm for tanks of an airplane

  Scenario Outline: Check expected total fuel left in both tanks
    Given the fuel level in the left tank is <fuel_left>L and in the right tank is <fuel_right>L
    When the simulation runs for <simulation_time> seconds
    Then the expected total fuel left should be around <expected_fuel_left>L
    Examples:
      | fuel_left | fuel_right | simulation_time | expected_fuel_left |
      | 100.0     | 100.0      | 60              | 42.0              |
      | 150.0     | 100.0      | 60              | 54.0              |
      | 200.0     | 200.0      | 120             | 18.0              |
      | 50.0      | 50.0       | 30              | 24.0               |
