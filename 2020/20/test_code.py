from unittest import TestCase
from .code import Tile, Direction
import numpy as np


class Day20Test(TestCase):
    def test_tile_placements(self):
        tile = Tile.from_string("""
Tile 1:
#.#
###
.##
""")
        np.testing.assert_equal(tile.border(Direction.NORTH), np.array([1, 0, 1]))
        np.testing.assert_equal(tile.border(Direction.SOUTH), np.array([0, 1, 1]))
        np.testing.assert_equal(tile.border(Direction.WEST), np.array([1, 1, 0]))
        np.testing.assert_equal(tile.border(Direction.EAST), np.array([1, 1, 1]))

    def test_tile_rotation(self):
        tile = Tile.from_string("""
Tile 1:
#.#
###
.##
""")
        np.testing.assert_equal(tile.rotate(1).data, np.array([[1, 1, 1], [0, 1, 1], [1, 1, 0]]))
        np.testing.assert_equal(tile.rotate(2).data, np.array([[1, 1, 0], [1, 1, 1], [1, 0, 1]]))

    def test_tile_flip(self):
        tile = Tile.from_string("""
Tile 1:
#.#
###
.##
""")
        np.testing.assert_equal(tile.flip(1, axis=1).data, np.array([[1, 0, 1], [1, 1, 1], [1, 1, 0]]))
        np.testing.assert_equal(tile.flip(1, axis=0).data, np.array([[0, 1, 1], [1, 1, 1], [1, 0, 1]]))

    def test_2d_convolution(self):
        monster_str = """                  # 
#    ##    ##    ###
 #  #  #  #  #  #   """
        monster_mask = np.array([
            np.array([
                1 if char == '#' else 0
                for char in line
            ])
            for line in monster_str.split("\n")
        ])
        from scipy.signal import convolve2d
        self.assertEqual(convolve2d(monster_mask, np.flip(monster_mask), mode='valid')[0][0], np.sum(monster_mask))
