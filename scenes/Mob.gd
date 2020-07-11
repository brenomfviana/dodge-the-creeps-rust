extends RigidBody2D

# Minimum speed range
export (int) var min_speed
# Maximum speed range
export (int) var max_speed
# Mob types
var mob_types = ["walk", "swim", "fly"]

func _ready():
	""" Called every time the node is added to the scene.
		Initialization here. """
	$AnimatedSprite.animation = mob_types[randi() % mob_types.size()]

func _on_Visibility_screen_exited():
	""" Free this node when it exits the screen """
	queue_free()
