Feature: Control algorithm for tanks of an airplane

      Scenario Outline: Prevent the tank from overflowing by switching between the left and right tank
            Given the fuel level in the left tank is <fuel_left>L and in the right tank is <fuel_right>L
            When the <overflow> tank reach <over_flow_level>
            And the simulation time <simulation_time> seconds
            Then switching the valve to <value_position>
            Examples:
                  | fuel_left | fuel_right | overflow | over_flow_level | simulation_time | value_position |
                  | 50.0      | 100.0      | right    | 50.0            | 60              | left           |
                  | 150.0     | 100.0      | left     | 50.0            | 60              | right          |
                  | 100.0     | 50.0       | left     | 50.0            | 60              | right          |
