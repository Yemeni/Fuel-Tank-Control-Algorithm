Feature: Control algorithm for tanks of an airplane

  # Testing where is the overflow
      Scenario Outline: The left tank should indicate that it's overflowed
            Given the fuel level in the left tank is <fuel_left>L and in the right tank is <fuel_right>L
            When 1 seconds pass
            Then the left tank should give <overflowed_left> and <overflowed_right> values according to the table
            Examples:
                  | fuel_left | fuel_right | overflowed_left | overflowed_right |
                  | 100.0     | 300.0      | false           | true             |
                  | 300.0     | 300.0      | true            | true             |
                  | 50.0      | 10.0       | false           | false            |
                  | 200.0     | 249.0      | true            | false            |

      Scenario Outline: Prevent the tank from overflowing by switching between the left and right tank
            Given the fuel level in the left tank is <fuel_left>L and in the right tank is <fuel_right>L
            When the <overflow> tank reach <over_flow_level>
            And the simulation time <simulation_time> seconds
            Then switching the valve to <valve_position>
            Examples:
                  | fuel_left | fuel_right | overflow | over_flow_level | simulation_time | valve_position |
                  | 50.0      | 100.0      | none     | 0.0             | 60              | right          |
                  | 150.0     | 100.0      | none     | 0.0             | 60              | left           |
                  | 100.0     | 50.0       | none     | 0.0             | 90              | left           |
                  | 200.1     | 50.0       | left     | 0.1             | 130             | left           |
                  | 200.1     | 250.1      | both     | 0.2             | 160             | right          |
                  | 0.0       | 250.1      | right    | 0.1             | 260             | right          |
