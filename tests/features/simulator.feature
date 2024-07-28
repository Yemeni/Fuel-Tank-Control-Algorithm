Feature: Simulation of tanks

  # Assumes left in-flow < out-flow
  Scenario Outline: Drain only left tank empty, short time
    Given the left tank is at <left> liter
	And the right tank is empty
	And the valve position is left
    When 3 seconds pass
    Then the left tank should be empty
	And the right tank should not be empty

    Examples: 
      | left | right |
      | 0    | 0     |
      | 10   | 0     |
      | 0    | 10    |
      | 10   | 10    |
      | 0    | 100   |
      | 10   | 100   |
      
  # Assumes left in-flow < out-flow
  Scenario Outline: Drain only left tank not empty, short time
    Given the left tank is at <left> liter
	And the right tank is empty
	And the valve position is left
    When 3 seconds pass
    Then the left tank should not be empty
	And the right tank should not be empty

    Examples: 
      | left | right |
      | 100  | 0     |
      | 200  | 0     |
      | 100  | 10    |
      | 200  | 10    |
      | 100  | 100   |
      | 200  | 100   |
	
  # Assumes right in-flow < out-flow
  Scenario Outline: Drain only right tank empty, short time
    Given the left tank is empty
	And the right tank is empty
	And the valve position is right
    When 3 seconds pass
    Then the right tank should be empty
	And the left tank should not be empty

    Examples: 
      | left | right |
      | 0    | 0     |
      | 0    | 10    |
      | 10   | 0     |
      | 10   | 10    |
      | 100  | 0     |
      | 100  | 10    |
      
  # Assumes right in-flow < out-flow
  Scenario Outline: Drain only right tank not empty, short time
    Given the left tank is empty
	And the right tank is empty
	And the valve position is right
    When 3 seconds pass
    Then the right tank should be empty
	And the left tank should not be empty

    Examples: 
      | left | right |
      | 0    | 100   |
      | 0    | 200   |
      | 10   | 100   |
      | 10   | 200   |
      | 100  | 100   |
      | 100  | 200   |
      
  # Assumes right in-flow is big enough to overflow after 1e3 seconds
  Scenario: Drain only left tank, long time
    Given the left tank is empty
	And the right tank is empty
	And the valve position is left
    When 1000 seconds pass
    Then the right tank should overflow

  # Assumes left in-flow is big enough to overflow after 1e3 seconds
  Scenario: Drain only right tank, long time
    Given the left tank is empty
	And the right tank is empty
	And the valve position is right
    When 1000 seconds pass
    Then the left tank should overflow
