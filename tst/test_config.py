"""
Unit tests for the Config class.
"""
import json
import os
import unittest

from src.config import Config
from src.language_information import LanguageInformation
from src.result import Ok, Error

example_config = {
    'foo': 'bar',
    'languages': {
        'python': {
            'executable': 'python3',
            'extension': '.py',
            'compiled': False
        }
    }
}


class TestConfig(unittest.TestCase):
    """
    Unit tests for the Config class.
    """

    @classmethod
    def setUpClass(cls):
        cls.temp_file_path = 'temp_config.json'

        with open(cls.temp_file_path, 'w', encoding='utf-8') as file_handler:
            json.dump(example_config, file_handler)

    @classmethod
    def tearDownClass(cls):
        os.remove(cls.temp_file_path)

    def test_01_initialize_with_file(self):
        """
        Verify that the config is loaded correctly.
        """
        # Act
        config = Config(self.temp_file_path)

        # Assert
        # pylint: disable=protected-access
        self.assertEqual(config.get_item('foo'), Ok('bar'))

    def test_02_initialize_without_file(self):
        """
        Verify that the config is not loaded
            when no path is passed when initially the Config is instanced
        """
        # Act
        Config.reset_config()
        config = Config()
        result = config.get_item('foo')

        # Assert
        self.assertEqual(result, Error('Config is not loaded'))

    def test_03_create_language_information(self):
        """
        Verify that the LanguageInformation object is created correctly.
        """
        # Arrange
        expected_executable = example_config['languages']['python']['executable']
        expected_extension = example_config['languages']['python']['extension']
        expected_compiled = example_config['languages']['python']['compiled']

        expected_language_information = LanguageInformation('python',
                                                            expected_executable,
                                                            expected_extension,
                                                            expected_compiled)
        config = Config(self.temp_file_path)

        # Act
        result = config.create_language_information('python')

        # Assert
        self.assertEqual(result, Ok(expected_language_information))

    def test_04_create_language_information_with_empty_config(self):
        """
        Verify that the LanguageInformation object is not created when the config is empty.
        """
        # Arrange
        Config.reset_config()
        config = Config()

        # Act
        result = config.create_language_information('python')

        # Assert
        self.assertEqual(result, Error('Config is not loaded'))

    def test_05_create_language_information_with_not_existing_language(self):
        """
        Verify that the LanguageInformation object is not created
            when the language is not in the config.
        """
        # Arrange
        config = Config(self.temp_file_path)

        # Act
        result = config.create_language_information('non_existing_language')

        # Assert
        self.assertEqual(result, Error('Language not found'))
